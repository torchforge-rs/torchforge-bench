# TODO

> Roadmap aligned to SemVer. `v0.x` = pre-alpha/alpha, no stability guarantees. `v1.0.0` = first stable published benchmark.
>
> `[RESEARCH]` — requires verification or prototyping before implementation.
> `[DECISION]` — blocked on an architectural choice in [ARCHITECTURE.md](ARCHITECTURE.md).

---

## Pre-v0.1.0 — Prerequisite Research

Must be resolved before any algorithm implementation begins. No exceptions.

### Environment Strategy — PyO3 FFI (decided)
- [ ] `[RESEARCH]` Measure PyO3 FFI overhead on environment `step()` call — quantify impact on timing measurements
- [ ] If overhead is negligible for CartPole: proceed with PyO3 for v0.1
- [ ] If overhead is significant: revisit native Rust CartPole as fallback (requires statistical equivalence test against Gymnasium)

### Neural Network Backend — Prototype Both, Expect burn
- [ ] `[RESEARCH]` Prototype minimal DQN Q-network + backward pass in `burn`+`ndarray`
- [ ] `[RESEARCH]` Prototype minimal DQN Q-network + backward pass in `candle`
- [ ] Compare: compile time, binary size, autodiff correctness, API ergonomics
- [ ] `[DECISION]` Commit to winner (expecting `burn`+`ndarray` for CartPole; reassess at v0.4 for GPU needs)

### Baselines & Infrastructure
- [ ] `[RESEARCH]` Establish Python CleanRL DQN baseline on target hardware — document results per measurement methodology in ARCHITECTURE.md

### Module Structure — Flat Algorithm-Centric Layout (decided)
```
src/
  lib.rs
  environment.rs    // Environment trait + PyO3 Gymnasium wrapper
  replay_buffer.rs  // Inline minimal buffer (replace with torchforge-data when available)
  measurement.rs    // Wall-clock, peak memory, JSON output
  algorithms/
    mod.rs
    dqn.rs          // Q-network, training loop
benches/
  dqn_cartpole.rs   // criterion micro-benchmarks + end-to-end
tests/
  environment_pyo3.rs  // PyO3 wrapper integration tests (fixed-trajectory assertions)
  dqn_convergence.rs   // #[ignore] statistical validation (≥475 mean return, 5 seeds)
```

### Unblocked — Inline Minimal Implementations
- [ ] Inline replay buffer (~50 lines, behind a trait) — replace with `torchforge-data` when it ships v0.1.0
- [ ] Inline scalar logging to JSON — replace with `torchforge-viz` when it ships v0.1.0

### Test Strategy
Each module gets `#[cfg(test)]` stubs when created. CI deferred until there's code to gate.

**Unit tests (run with `cargo test`):**
- `replay_buffer.rs` — capacity enforcement, FIFO eviction, sampling uniformity, empty-buffer edge case
- `measurement.rs` — wall-clock monotonically increases, peak memory returns nonzero on Linux

**Integration tests (in `tests/`):**
- `environment_pyo3.rs` — run a fixed action sequence through the PyO3 Gymnasium wrapper and assert identical observations/rewards/done signals vs. known CartPole trajectory (catches FFI serialization bugs, type mismatches, state divergence)

**Statistical validation tests (`#[test] #[ignore]`, long-running):**
- `dqn_convergence.rs` — DQN on CartPole-v1 must achieve mean episode return ≥ 475 over last 100 episodes, averaged over 5 seeds. Run manually or in a dedicated CI slow-test job.

### Performance Strategy
Decisions to avoid costly rework once implementation begins.

**Dual-timer instrumentation:**
- Every `env.step()` and `env.reset()` call wrapped with `Instant::now()` timing
- Report both: total wall-clock (apples-to-apples with CleanRL) and training-only (excluding env FFI)
- ARCHITECTURE.md methodology table updated to include "Training time (excluding env)" field

**Peak memory measurement:**
- `/proc/self/status` (`VmHWM`) on Linux — sufficient for published benchmarks (all run on Linux)
- Returns `None` on non-Linux platforms; tests skip the assertion
- No cross-platform crate dependency needed

**Replay buffer implementation:**
- Simple `Vec<Transition>` with `rand::seq::index::sample` — sufficient for CartPole's small buffer
- `criterion` micro-benchmark for sample operation from day one — provides data for optimization at v0.2+

**Benchmark hygiene checklist (document in ARCHITECTURE.md):**
- Set CPU governor to `performance` (disable frequency scaling)
- Close unnecessary processes
- Run 1 warmup seed before 5 measured seeds
- Report system load alongside results
- On Raspberry Pi: monitor thermal throttling (`vcgencmd get_throttled`)

---

## v0.1.0 — DQN on CartPole-v1

**Goal**: Match CleanRL's DQN on CartPole-v1. Publish results — including if we are slower.

### Infrastructure
- [ ] Benchmark harness: multi-seed runner with structured JSON output
- [ ] JSON results schema (defined and documented)
- [ ] Results comparison tool: Rust results vs. stored Python baseline
- [ ] `criterion` integration for micro-benchmarks (replay buffer sample, forward pass)

### DQN Implementation
- [ ] Q-network MLP (input: observation dim, output: action dim)
- [ ] Target network with periodic hard update
- [ ] Epsilon-greedy exploration with linear decay
- [ ] Replay buffer integration (inline impl; swap to `torchforge-data` when available)
- [ ] Adam optimizer
- [ ] Huber loss / MSE loss (configurable)
- [ ] Training loop matching CleanRL's `dqn.py` structure exactly

### CartPole-v1 Environment
- [ ] PyO3 Gymnasium wrapper implementing `Environment` trait
- [ ] Verify `CartPole-v1` reset/step/render work through FFI

### Validation
- [ ] Verify episode return convergence matches CleanRL baseline within statistical tolerance (mean ± std over 5 seeds)
- [ ] Verify dual-timer correctness: total wall-clock and training-only (excluding env FFI) both reported
- [ ] Verify peak memory measurement via `/proc/self/status` (`VmHWM`) on Linux
- [ ] Run benchmark hygiene checklist before final results collection

### Results Publication
- [ ] Publish results in `results/dqn_cartpole/` with full methodology
- [ ] Document hardware used
- [ ] If slower than Python: document why, do not hide

---

## v0.2.0 — PPO on CartPole-v1

**Goal**: Extend to an on-policy algorithm. PPO is the most widely used RL algorithm in practice.

- [ ] `[RESEARCH]` Establish CleanRL PPO baseline on same hardware
- [ ] Rollout buffer (different from replay buffer — on-policy)
- [ ] GAE (Generalized Advantage Estimation)
- [ ] Policy network + value network (shared or separate)
- [ ] PPO clipping loss
- [ ] Entropy bonus
- [ ] Mini-batch updates over collected rollouts
- [ ] Training loop matching CleanRL's `ppo.py` structure
- [ ] Publish results

---

## v0.3.0 — Edge Hardware Benchmarks

**Goal**: Run DQN and PPO on real edge hardware. This is the core claim of the torchforge ecosystem.

- [ ] `[RESEARCH]` Identify minimum viable edge target (Raspberry Pi 5 is current candidate)
- [ ] Port and verify builds cross-compile cleanly for ARM
- [ ] Run DQN benchmark on edge target
- [ ] Run PPO benchmark on edge target
- [ ] Document memory usage vs. Python equivalent (Python cannot run on the same target without significant overhead)
- [ ] Publish edge results separately from workstation results

---

## v0.4.0 — SAC on Continuous Control

**Goal**: Extend to continuous action spaces. Required for real-world robotics applicability.

- [ ] `[RESEARCH]` Identify suitable continuous control environment driveable from Rust
- [ ] `[RESEARCH]` Establish CleanRL SAC baseline
- [ ] Actor network (continuous, squashed Gaussian policy)
- [ ] Twin Q-networks
- [ ] Automatic entropy tuning
- [ ] Soft target network updates
- [ ] Publish results

---

## v1.0.0 — First Stable Benchmark Publication

**Gate criteria** (all must be met):
- [ ] DQN, PPO results published with full methodology on at least one workstation
- [ ] At least one edge hardware result published
- [ ] Results reviewed by at least one external contributor for methodology correctness
- [ ] All benchmarks reproducible from a single `cargo bench` command
- [ ] Python baselines stored in repo and reproducible from a single `uv run` command
- [ ] MSRV declared and tested

---

## Ongoing

- [ ] Keep dependencies on latest stable versions
- [ ] `cargo audit` clean at all times
- [ ] `cargo clippy -- -D warnings` clean at all times
- [ ] CHANGELOG.md maintained per [Keep a Changelog](https://keepachangelog.com/)
- [ ] Never publish a result without the full methodology table from ARCHITECTURE.md
