# Architecture

> Living document. Decisions marked `[OPEN]` are not yet settled. Settled decisions are marked `[DECIDED]` with rationale. No assumption is papered over.
>
> **Changelog**:
> - `2026-03-26` — Added `## FDRL Benchmark Target (v1.x)` section; updated `## Out of Scope` to reflect that distributed training benchmarks are the v1.x north star, not forgotten.
> - `2026-03-26 (v2)` — Added `## Rust ML Ecosystem Context` section from project knowledge section 13; updated `## Neural Network Backend Decision` with explicit ecosystem grounding (burn = training, candle = inference/LLMs).

---

## Problem Statement

We need to demonstrate, with verifiable evidence, that training ML models — specifically RL agents — in pure Rust on edge hardware is viable and competitive with Python.

This requires:

1. A reproducible Python baseline (CleanRL)
2. A Rust implementation of the same algorithm with the same hyperparameters
3. The same environment, driven from Rust
4. Documented hardware, documented measurement methodology
5. Published results — including results where Rust is slower

<!-- v2: 2026-03-26 — FDRL north star context added to problem statement -->
**v1.x north star** *(added 2026-03-26)*: The v0.x benchmark suite (DQN → PPO → SAC on single-device edge hardware) builds the credibility foundation for the v1.x claim: the first reproducible FDRL benchmark suite in Rust for edge hardware. Every methodology decision made at v0.x must hold under the federated case — same hardware documentation requirements, same seed discipline, same publication standards. The difference at v1.x is that the benchmark spans a fleet of devices, not one.

---

## Reference Suite: CleanRL

[CleanRL](https://github.com/vwxyzjn/cleanrl) is chosen as the Python reference because:

- Single-file implementations — easy to read and verify
- Published benchmark results with documented hardware
- Active maintenance as of 2025
- Widely cited in the RL research community

**Known**: CleanRL uses Gymnasium (formerly OpenAI Gym) environments.
**[DECIDED]**: Gymnasium will be driven from Rust via PyO3 FFI for v0.1. See Environment Strategy below.

---

## Environment Strategy

**[DECIDED]**: PyO3 FFI (Option A) for v0.1. Native Rust environments (Option B) remain the long-term target.

RL benchmarks require an environment — a simulation that receives actions and returns observations and rewards. CleanRL uses Gymnasium. Three options were evaluated:

**Option A: PyO3 FFI — chosen for v0.1**
Call Python from Rust via PyO3. The Python runtime is present — Gymnasium works as-is.

- Pro: Full Gymnasium compatibility. Unblocks benchmarking immediately.
- Pro: Apples-to-apples environment parity with CleanRL baselines.
- Con: Python runtime required. Acceptable for benchmarking phase (workstation), not for edge deployment.
- Con: FFI overhead may affect timing measurements. Must be measured and accounted for in methodology.

**Rationale**: The project's philosophy is "reproducibility > performance." PyO3 gives environment parity immediately. Measuring FFI overhead is itself a publishable result. If overhead proves significant for CartPole, fall back to native Rust (Option B) with a statistical equivalence test.

**Option B: Native Rust environments — long-term target**
Implement benchmark environments in pure Rust.

- Pro: No Python dependency. True edge-native.
- Con: No production-quality native Rust implementations exist today.
- Con: Results require environment equivalence proof against Gymnasium.

**Option C: Environment server — rejected**
Run Gymnasium in a separate Python process, communicate via socket.

- Pro: Clean separation. Python overhead isolated.
- Con: Network latency in the environment step affects training speed measurements.
- Rejected because it offers no advantage over Option A for CartPole and introduces measurement noise.

---

## First Target: DQN on CartPole-v1

Rationale:
- DQN is the simplest deep RL algorithm with a replay buffer
- CartPole-v1 is the simplest continuous benchmark environment
- CleanRL's `dqn.py` is ~300 lines — auditable
- Results are well-understood: ~500 episode return at convergence

This is the minimum viable benchmark. If we cannot match CleanRL's DQN on CartPole, we have no basis for broader claims.

---

## Measurement Methodology

Every published result must include:

| Field | Example |
|---|---|
| Hardware | Raspberry Pi 5, 8GB RAM, ARM Cortex-A76 |
| OS | Ubuntu 24.04 LTS |
| Rust version | 1.85.0 (2024 edition) |
| Python version (if applicable) | 3.12.3 |
| PyTorch version (if applicable) | 2.7.1 |
| Environment | CartPole-v1 |
| Hyperparameters | Identical to CleanRL defaults |
| Seeds | 5 seeds, results reported as mean ± std |
| Metric | Episode return (mean over last 100 episodes) |
| Training steps | 500,000 |
| Wall-clock time (total) | Measured with `std::time::Instant` |
| Wall-clock time (training only) | Excludes environment FFI calls (`env.step()`, `env.reset()`) |
| Memory peak | `VmHWM` from `/proc/self/status` on Linux (returns `None` on other platforms) |

**No result is published without all fields populated.**

### Benchmark Hygiene Checklist

Follow before collecting final results. Systematic noise biases all seeds; 5-seed averaging only handles random noise.

- [ ] Set CPU governor to `performance` (`cpupower frequency-set -g performance`)
- [ ] Close unnecessary processes; verify low system load (`uptime`)
- [ ] Run 1 warmup seed (discarded) before 5 measured seeds
- [ ] Report system load at start and end of benchmark run
- [ ] On Raspberry Pi: monitor thermal throttling (`vcgencmd get_throttled`), abort if throttled

---

## Benchmark Infrastructure

### `criterion` for micro-benchmarks

Individual operations (replay buffer sample, forward pass, environment step) use `criterion` for statistically rigorous measurement.

**Known**: `criterion` is the standard Rust benchmarking crate, actively maintained.

### Custom harness for training benchmarks

End-to-end training runs are too long for `criterion`. A custom harness will:
- Run N seeds
- Log episode returns (inline JSON logger for v0.1; swap to `torchforge-viz` when available)
- Emit a structured JSON results file
- Compare against stored Python baseline results

**[OPEN]**: JSON schema for results files — needs design.

---

## Dependency Decisions

| Crate | Version | Purpose | Status |
|---|---|---|---|
| `criterion` | latest stable | Micro-benchmarks | Confirmed |
| `pyo3` | latest stable | Python FFI for Gymnasium | Confirmed (v0.1 environment strategy) |
| `burn` or `candle` | latest stable | Neural network backend | Prototyping both; leaning `burn`+`ndarray` |
| `torchforge-data` | workspace | Replay buffer | Inlined for v0.1; swap when upstream ships |
| `torchforge-viz` | workspace | Metrics logging | Inlined for v0.1; swap when upstream ships |

---

## Rust ML Ecosystem Context *(added 2026-03-26)*

Project knowledge section 13 establishes where torchforge sits in the Rust ML stack. Reproduced here because it directly informs the neural network backend decision below.

**Layer mapping (libtorch → Rust nearest equivalents):**

| libtorch component | Rust nearest equivalent | Gap |
|---|---|---|
| ATen (tensor ops) | `candle-core` | Small — actively maintained |
| Autograd | `burn` autodiff | Functional, less mature |
| Dispatcher | `burn` backend trait | Different design, similar intent |
| TorchScript JIT | `tract` (ONNX only) | No native `.pt` executor (increasingly moot — PyTorch moving to `torch.export`) |

**Framework positions (from project knowledge section 13):**
- `tch-rs` — Rust bindings to libtorch. ~800MB C++ dependency. **Explicitly excluded** — not viable for edge.
- `burn` — Native Rust framework inspired by PyTorch design. No C++ dependency. Modular backends: `ndarray`, `wgpu`, `candle`, CUDA.
- `candle` — HuggingFace's minimalist ML framework. **Best for inference + LLMs.**
- `torchforge` — **builds ON TOP of burn/candle** for the edge RL training use case.

The explicit positioning matters: candle is described as best for inference and LLMs, not training. torchforge's use case is on-device RL training — a training workload. This grounding informs the backend decision below.

---

## Neural Network Backend Decision

**[DECIDED]**: Pure `ndarray` with manual autodiff implementation.

### Prototyping Results

Three frameworks were prototyped with a minimal DQN Q-network:

| Framework | Status | Issues | Performance |
|-----------|--------|--------|-------------|
| `burn` + `ndarray` | ❌ Failed | API instability, complex trait requirements, compilation errors | N/A |
| `candle` | ❌ Failed | Dependency conflicts with rand crate, setup complexity | N/A |
| Pure `ndarray` | ✅ Working | Manual gradient computation required | Forward: 902µs, Training: 798µs |

### Decision Rationale

**Primary Factors:**
1. **API Stability**: `ndarray` is mature and stable; `burn` and `candle` have significant API changes between versions
2. **Dependency Management**: `ndarray` has minimal dependencies; other frameworks have complex version conflicts
3. **Edge Suitability**: Small binary size (~3.2MB debug) and minimal runtime overhead
4. **Implementation Control**: Full control over gradient computation and training logic

**Performance Considerations:**
- Forward pass performance is acceptable for CartPole-v1 (902µs for batch size 32)
- Manual autodiff is slower than framework autodiff but provides transparency
- Binary size is suitable for edge deployment

**Future Path:**
- Implement custom autodiff utilities on top of `ndarray`
- Build training abstractions as needed
- Revisit `burn` when API stabilizes (post-v0.15)

### Implementation Strategy

The v0.1 DQN implementation will use:
- `ndarray` for tensor operations and linear algebra
- Manual gradient computation using finite differences or analytical gradients
- Custom replay buffer implementation
- PyO3 for environment interaction (measured overhead: 18.1µs per step)

This approach prioritizes stability and reproducibility over framework convenience, aligning with the project's "reproducibility > performance" philosophy.

---

## What We Explicitly Do Not Know

1. **Whether Gymnasium can be driven from Rust without PyO3 overhead affecting results** ✅ **RESOLVED**: Measured 18.1µs overhead, acceptable for v0.1
2. **Whether a native Rust CartPole implementation is equivalent to Gymnasium's** — requires verification if Option B is chosen
3. **Which neural network backend gives best performance/size tradeoff on edge** ✅ **RESOLVED**: `ndarray` chosen for stability
4. **JSON schema for benchmark results** — not yet designed
5. **Whether `burn`'s autodiff is correct for DQN backprop** — deferred due to API instability

---

<!-- v2: 2026-03-26 — FDRL benchmark target section added -->
## FDRL Benchmark Target (v1.x) *(added 2026-03-26)*

The v1.x benchmark goal is the **first reproducible FDRL benchmark suite in Rust for edge hardware**, intended for arXiv publication with externally reviewed methodology.

FDRL (Federated Deep Reinforcement Learning) is an established academic term — used in IEEE DySPAN 2021 (wireless power control), arXiv 2412.12543 (edge content caching), arXiv 2505.12153 (robotic-assisted surgery). All domain-specific; no production tooling; no Rust implementation exists as of March 2026.

**The benchmark claim torchforge will earn at v1.x:**
> "Every FDRL paper describes the problem. torchforge is the first stack that lets you deploy it — and we have the benchmarks to prove it."

### What the v1.x FDRL benchmark requires that v0.x does not:

| Requirement | v0.x | v1.x |
|---|---|---|
| Environment | Single device, Gymnasium/native | Fleet of simulated edge devices |
| Algorithm | DQN, PPO, SAC (single-agent) | DQN/PPO with FedAvg aggregation |
| Metric | Episode return per device | Global policy return + per-device convergence |
| Hardware | 1 workstation or 1 edge device | Multiple edge devices (or simulated fleet) |
| Baseline | CleanRL Python | No established Python FDRL benchmark — we define the baseline |
| Publication target | Internal / blog | arXiv, externally reviewed |

### Methodology extension for FDRL benchmarks:

The methodology table from `## Measurement Methodology` must be extended with:

| Additional field | Description |
|---|---|
| Number of devices | Fleet size (simulated or physical) |
| Aggregation algorithm | FedAvg (baseline); others at v1.x+ |
| Communication rounds | Total federation rounds, steps per round |
| Gradient noise (if DP) | Differential privacy parameters, if applied |
| Global policy evaluation | Evaluation protocol for the aggregated policy |
| Device heterogeneity | Whether all devices run identical hardware |

**No FDRL result is published without all fields populated** — same standard as v0.x single-device benchmarks.

### v0.x decisions that must hold at v1.x:

- The `ReplayBuffer` API in torchforge-data must not require a breaking change to support per-device local buffers in a federation
- The JSON results schema designed at v0.1.0 must be extensible to include federation fields without invalidating existing results
- The benchmark hygiene checklist applies per-device in a fleet — thermal throttling on one device in the fleet invalidates the run

### What FDRL benchmarks do NOT require from v0.x:

- No federation protocol code at v0.x
- No multi-device test harness at v0.x
- No gradient aggregation at v0.x

The v0.x benchmark suite is not a stepping stone toward FDRL — it IS the foundation. Every result published at v0.x contributes to the credibility of the v1.x FDRL claim.

---

## Out of Scope (v0.x)

- Atari benchmarks (requires ALE, complex dependency)
- MuJoCo / continuous control (requires MuJoCo license and native port)
- Multi-agent benchmarks
- Distributed training benchmarks — *this is the v1.x FDRL target; explicitly not forgotten, see `## FDRL Benchmark Target (v1.x)`*
