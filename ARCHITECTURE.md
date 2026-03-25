# Architecture

> Living document. Decisions marked `[OPEN]` are not yet settled. Settled decisions are marked `[DECIDED]` with rationale. No assumption is papered over.

---

## Problem Statement

We need to demonstrate, with verifiable evidence, that training ML models — specifically RL agents — in pure Rust on edge hardware is viable and competitive with Python.

This requires:

1. A reproducible Python baseline (CleanRL)
2. A Rust implementation of the same algorithm with the same hyperparameters
3. The same environment, driven from Rust
4. Documented hardware, documented measurement methodology
5. Published results — including results where Rust is slower

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

## Neural Network Backend Decision

DQN requires a small MLP (2-3 layers). The backend choice affects:
- Compile time
- Binary size (critical for edge)
- GPU availability
- Maintenance burden

Candidates:
- `burn` with `ndarray` backend — pure Rust, no C++ dependency, CPU-only
- `candle` — HuggingFace, actively maintained, Metal/CUDA support
- `tch-rs` — full PyTorch, but ~800MB dependency (unacceptable for edge)

**[DECIDED]**: Prototype both `burn`+`ndarray` and `candle` with a minimal DQN Q-network + backward pass. Compare compile time, binary size, autodiff correctness, and API ergonomics. Expecting `burn`+`ndarray` to win for CartPole (no GPU needed). Reassess at v0.4 when GPU support may matter for continuous control.

---

## What We Explicitly Do Not Know

1. **Whether Gymnasium can be driven from Rust without PyO3 overhead affecting results** — needs measurement
2. **Whether a native Rust CartPole implementation is equivalent to Gymnasium's** — requires verification if Option B is chosen
3. **Which neural network backend gives best performance/size tradeoff on edge** — requires prototyping
4. **JSON schema for benchmark results** — not yet designed
5. **Whether `burn`'s autodiff is correct for DQN backprop** — requires validation against known-correct Python results

---

## Out of Scope (v0.x)

- Atari benchmarks (requires ALE, complex dependency)
- MuJoCo / continuous control (requires MuJoCo license and native port)
- Multi-agent benchmarks
- Distributed training benchmarks
