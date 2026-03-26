# TODO

> Roadmap aligned to SemVer. `v0.x` = pre-alpha/alpha, no stability guarantees. `v1.0.0` = first stable published benchmark.
>
> `[RESEARCH]` — requires verification or prototyping before implementation.
> `[DECISION]` — blocked on an architectural choice in [ARCHITECTURE.md](ARCHITECTURE.md).
>
> **Changelog**:
> - `2026-03-26` — Added `## v1.x — FDRL Benchmark Suite` section; noted JSON schema extensibility requirement for federation fields; updated `results/` append-only policy to include FDRL result files.
> - `2026-03-26 (v2)` — Updated `### Neural Network Backend` prototype items with project knowledge section 13 ecosystem grounding (burn = training, candle = inference/LLMs, tch-rs explicitly excluded).

---

## Phase 0 — Project Foundation

**Goal**: Establish the repository as a credible, contribution-ready OSS project before any functional code ships. These items are prerequisites for everything below — nothing is merged to `main` until Phase 0 is complete.

### Repository Structure
- [ ] Initialize repository with standard layout:
  ```
  .github/
    workflows/
    ISSUE_TEMPLATE/
    PULL_REQUEST_TEMPLATE.md
    CODEOWNERS
  baselines/          <- Python CleanRL baseline scripts (managed via uv)
    pyproject.toml
    uv.lock
    dqn_cartpole.py   <- CleanRL dqn.py, unmodified or minimally patched
    README.md         <- documents how to reproduce baselines
  benches/
  results/            <- published benchmark results (JSON + methodology tables)
    .gitkeep
  src/
  tests/
  ARCHITECTURE.md
  CHANGELOG.md
  CODE_OF_CONDUCT.md
  CONTRIBUTING.md
  LICENSE
  README.md
  SECURITY.md
  TODO.md
  ```
- [ ] `Cargo.toml` with correct metadata: `name`, `version = "0.0.1"`, `edition = "2024"`, `rust-version = "1.85"`, `license = "Apache-2.0"`, `repository`, `homepage`, `description`, `keywords`, `categories`
- [ ] `.gitignore` (Rust + Python `__pycache__`, `.venv`, `results/*.json` if results are large — decide policy)
- [ ] `rust-toolchain.toml` pinning `stable` channel

### License
- [ ] `LICENSE` — Apache-2.0 full text
- [ ] SPDX identifier `Apache-2.0` in `Cargo.toml`
- [ ] `baselines/` contains CleanRL code — CleanRL is MIT licensed; document this clearly in `baselines/README.md` and ensure attribution is correct. The two licenses are compatible but the distinction must be explicit.

### Governance Documents
- [ ] `CODE_OF_CONDUCT.md` — Contributor Covenant v2.1
- [ ] `CONTRIBUTING.md` — must cover:
  - Prerequisites: Rust 1.85+, Python 3.12+ via `uv`, Gymnasium dependencies (`pip install gymnasium[classic-control]`)
  - How to build and run Rust tests locally
  - How to run Python baselines locally (`uv run python baselines/dqn_cartpole.py`)
  - Branching model and PR process
  - Commit message format (Conventional Commits recommended)
  - Code style: `cargo fmt`, `cargo clippy -- -D warnings`
  - What "ready to merge" means for benchmark results: full methodology table populated, hygiene checklist completed, results file committed to `results/`
  - Policy on result reproducibility: any PR touching algorithm implementations must re-run the affected benchmark and update `results/`
  - Issue templates for bug reports, methodology challenges, feature requests
- [ ] `SECURITY.md` — must cover:
  - Supported versions (latest `v0.x` only)
  - Private reporting via GitHub Security Advisories
  - Response SLA: acknowledge within 72 hours, triage within 7 days
  - In scope: soundness issues in `unsafe` blocks, supply chain issues
  - Note: benchmark result disputes are not security issues — open a regular issue

### GitHub Templates
- [ ] `.github/PULL_REQUEST_TEMPLATE.md` — checklist: description, linked issue, algorithm changes re-benchmarked, methodology table populated, results file updated, CHANGELOG entry
- [ ] `.github/ISSUE_TEMPLATE/bug_report.md` — Rust version, Python version, OS, hardware, reproduction steps
- [ ] `.github/ISSUE_TEMPLATE/methodology_challenge.md` — for disputing a published result: what is incorrect, proposed correction, supporting evidence
- [ ] `.github/ISSUE_TEMPLATE/feature_request.md` — algorithm or environment request with rationale
- [ ] `CODEOWNERS` — assign owners to `ARCHITECTURE.md`, `results/`, `baselines/`, `SECURITY.md`, `Cargo.toml`

### CI — GitHub Actions
- [ ] `ci.yml` — runs on every push and PR to `main`:
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings`
  - `cargo test` (unit + integration tests; statistical convergence tests are `#[ignore]`)
  - `cargo doc --no-deps`
  - Matrix: `[stable, nightly]` x `[ubuntu-latest]`
- [ ] `audit.yml` — push to `main` + daily schedule:
  - `cargo audit`
  - `cargo deny check`
- [ ] `baselines.yml` — manual trigger (`workflow_dispatch`) only:
  - Sets up Python via `uv`, installs CleanRL dependencies
  - Runs `baselines/dqn_cartpole.py` and uploads results as workflow artifact
  - Not a CI gate — baselines are hardware-sensitive; run manually when establishing a new environment target
- [ ] `bench.yml` — manual trigger (`workflow_dispatch`) only:
  - `cargo bench` — criterion micro-benchmarks
  - Uploads results as workflow artifact
- [ ] Cache `~/.cargo/registry` and `target/` across workflow runs
- [ ] Branch protection on `main`: all `ci.yml` checks required, at least one review required

### Python Baseline Infrastructure
- [ ] `baselines/pyproject.toml` managed via `uv`:
  - Pin exact versions: `torch`, `gymnasium[classic-control]`, `cleanrl` (or inline script)
  - `uv.lock` committed — reproducible installs for anyone cloning the repo
- [ ] `baselines/README.md` documenting:
  - How to reproduce each baseline (`uv run python baselines/dqn_cartpole.py`)
  - License attribution for CleanRL code
  - Hardware used for published baseline numbers
  - How results map to the JSON schema in `results/`

### Results Directory
- [ ] `results/README.md` documenting the JSON schema for benchmark results files
- [ ] Schema must include all fields from the methodology table in ARCHITECTURE.md (hardware, OS, versions, seeds, metrics, wall-clock, memory)
- [ ] *(added 2026-03-26)* Schema must be designed for extensibility — federation fields (device count, aggregation algorithm, communication rounds) must be addable at v1.x without invalidating existing v0.x result files. Use optional fields with explicit null rather than absent keys.
- [ ] Policy: `results/` is append-only — never overwrite a published result, add a new dated file instead

### Changelog
- [ ] `CHANGELOG.md` initialized per [Keep a Changelog](https://keepachangelog.com/) format
- [ ] Policy: every PR requires a CHANGELOG entry; benchmark result updates are entries under `[Added]`

### Supply Chain
- [ ] `deny.toml` for `cargo deny`: license allowlist, bans on duplicate crates, advisory denies
- [ ] Note: Python dependencies in `baselines/` are not covered by `cargo deny` — `uv.lock` provides the equivalent guarantee for the Python side

### README Polish
- [ ] Badges rendering correctly: crates.io, docs.rs, license, CI status
- [ ] Prerequisites section: Rust, Python, `uv`
- [ ] "Results" section links to `results/` once first benchmark is published
- [ ] Links to `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`

---

## Pre-v0.1.0 — Prerequisite Research

**These are hard blockers. No algorithm implementation begins until all are resolved.**

### Environment Strategy — PyO3 FFI (decided for v0.1)
- [ ] `[RESEARCH]` Measure PyO3 FFI overhead on CartPole-v1 `env.step()` — run fixed-length episodes with a no-op policy, measure per-step latency in ns. If overhead exceeds ~10% of expected training-only step time, revisit native Rust fallback.
- [ ] Document FFI overhead as a published measurement in `results/ffi_overhead/` with full methodology

### Neural Network Backend
- [ ] `[RESEARCH]` Prototype minimal DQN Q-network + forward + backward pass in `burn`+`ndarray` — project knowledge section 13 positions `burn` as the training framework; torchforge "builds ON TOP of burn/candle for the edge RL training use case"; this is the primary candidate
- [ ] `[RESEARCH]` Prototype minimal DQN Q-network + forward + backward pass in `candle` — project knowledge section 13 explicitly positions candle as "best for inference + LLMs"; prototype regardless to confirm the ecosystem framing holds for a training workload
- [ ] Compare: compile time, binary size, autodiff correctness vs. known PyTorch output, API ergonomics — expect `burn`+`ndarray` to win given section 13 positioning, but do not skip the comparison
- [ ] `[DECISION]` Commit to winner — document in ARCHITECTURE.md; if `candle` outperforms despite section 13 framing, document the discrepancy honestly
- [ ] Note: `tch-rs` is **explicitly excluded** per project knowledge sections 9 and 13 — ~800MB libtorch dependency, not viable for edge; do not prototype

### Python Baseline Measurement
- [ ] `[RESEARCH]` Run CleanRL `dqn.py` on target hardware, collect results per methodology table in ARCHITECTURE.md
- [ ] Store results in `results/baselines/dqn_cartpole_python_<hardware>.json`
- [ ] This is the reference every Rust result will be measured against — it must exist before the Rust implementation begins

---

## v0.1.0 — DQN on CartPole-v1

**Goal**: Match CleanRL's DQN on CartPole-v1. Publish results — including if we are slower.

### Infrastructure
- [ ] Module structure per ARCHITECTURE.md:
  - `src/environment.rs` — `Environment` trait + PyO3 Gymnasium wrapper
  - `src/replay_buffer.rs` — inline minimal buffer (replace with `torchforge-data` when available)
  - `src/measurement.rs` — wall-clock, peak memory (`/proc/self/status` `VmHWM`), JSON output
  - `src/algorithms/dqn.rs` — Q-network, training loop
- [ ] Benchmark harness: multi-seed runner with structured JSON output
- [ ] JSON results schema (defined in `results/README.md`, implemented here)
- [ ] Results comparison tool: Rust results vs. stored Python baseline
- [ ] `criterion` micro-benchmarks: replay buffer sample, forward pass, environment step

### DQN Implementation
- [ ] Q-network MLP (input: observation dim, output: action dim)
- [ ] Target network with periodic hard update
- [ ] Epsilon-greedy exploration with linear decay
- [ ] Adam optimizer
- [ ] Huber loss (configurable to MSE)
- [ ] Training loop matching CleanRL `dqn.py` structure and hyperparameters exactly

### CartPole-v1 Environment
- [ ] PyO3 Gymnasium wrapper implementing `Environment` trait
- [ ] `env.reset()`, `env.step()`, `env.close()` via FFI
- [ ] Fixed-trajectory integration test: run known action sequence, assert observations/rewards/done match expected CartPole behavior (catches FFI serialization bugs and type mismatches)

### Dual-Timer Instrumentation
- [ ] Every `env.step()` and `env.reset()` wrapped with `Instant::now()` timing
- [ ] Report both: total wall-clock and training-only (excluding env FFI calls)
- [ ] Both fields required in published results JSON

### Validation
- [ ] Statistical convergence test (`#[test] #[ignore]`): DQN on CartPole-v1 must achieve mean episode return >= 475 over last 100 episodes, averaged over 5 seeds
- [ ] Verify peak memory measurement via `/proc/self/status` `VmHWM` on Linux (skip assertion on non-Linux)
- [ ] Run benchmark hygiene checklist (ARCHITECTURE.md) before final results collection

### Results Publication
- [ ] Publish results in `results/dqn_cartpole/` with full methodology table
- [ ] If slower than Python: document gap, hypothesize cause, do not hide

---

## v0.2.0 — PPO on CartPole-v1

**Goal**: Extend to an on-policy algorithm.

- [ ] `[RESEARCH]` Run CleanRL PPO baseline on same hardware, store results
- [ ] Rollout buffer (on-policy — different from replay buffer)
- [ ] GAE (Generalized Advantage Estimation)
- [ ] Policy network + value network
- [ ] PPO clipping loss
- [ ] Entropy bonus
- [ ] Mini-batch updates over collected rollouts
- [ ] Training loop matching CleanRL `ppo.py` structure and hyperparameters exactly
- [ ] Publish results

---

## v0.3.0 — Edge Hardware Benchmarks

**Goal**: Run DQN and PPO on real edge hardware. This is the core claim of the torchforge ecosystem.

- [ ] `[RESEARCH]` Identify minimum viable edge target (Raspberry Pi 5 is current candidate)
- [ ] Verify cross-compilation to ARM builds cleanly
- [ ] Run DQN benchmark on edge target — document memory usage vs. Python equivalent
- [ ] Run PPO benchmark on edge target
- [ ] Publish edge results separately from workstation results (different hardware = different result file)
- [ ] Thermal throttling monitoring required on Raspberry Pi (`vcgencmd get_throttled`) — abort run if throttled

---

## v0.4.0 — SAC on Continuous Control

**Goal**: Extend to continuous action spaces.

- [ ] `[RESEARCH]` Identify suitable continuous control environment driveable from Rust
- [ ] `[RESEARCH]` Run CleanRL SAC baseline on same hardware
- [ ] Actor network (continuous, squashed Gaussian policy)
- [ ] Twin Q-networks
- [ ] Automatic entropy tuning
- [ ] Soft target network updates
- [ ] Publish results

---

## v1.0.0 — First Stable Benchmark Publication

**Gate criteria** (all must be met):
- [ ] DQN and PPO results published with full methodology on at least one workstation
- [ ] At least one edge hardware result published
- [ ] Results reviewed by at least one external contributor for methodology correctness
- [ ] All benchmarks reproducible from a single `cargo bench` command
- [ ] Python baselines reproducible from a single `uv run` command
- [ ] MSRV declared and tested in CI
- [ ] JSON results schema verified extensible for federation fields (FDRL forward compatibility)

---

## v1.x — FDRL Benchmark Suite *(added 2026-03-26)*

**Goal**: First reproducible FDRL benchmark suite in Rust for edge hardware. Publication target: arXiv, externally reviewed methodology.

FDRL (Federated Deep Reinforcement Learning) has academic precedent (IEEE DySPAN 2021, arXiv 2412.12543, arXiv 2505.12153) but no production tooling and no Rust implementation as of March 2026. This is not a new concept we coin — it is a gap in tooling we fill.

See ARCHITECTURE.md `## FDRL Benchmark Target (v1.x)` for full methodology extension, metric table, and what distinguishes FDRL benchmarks from v0.x single-device benchmarks.

### Prerequisites (must complete before FDRL benchmarks begin)
- [ ] torchforge-data v1.0.0 stable — per-device `ReplayBuffer` API locked, `Transition: Send + Sync` confirmed
- [ ] torchforge-viz v1.0.0 stable — per-device and global policy logging, `local/` vs. `global/` tag namespace confirmed
- [ ] torchforge-federated crate created — gradient aggregation (FedAvg), communication protocol
- [ ] FDRL extended methodology table finalized and documented in ARCHITECTURE.md

### v1.x Benchmark Items
- [ ] `[RESEARCH]` Identify FDRL Python reference — no CleanRL equivalent exists; define reference or adapt domain-specific research code; document the choice explicitly
- [ ] `[DECISION]` Simulated fleet vs. physical device fleet for initial v1.x benchmarks
- [ ] DQN + FedAvg on CartPole-v1 (simulated N-device fleet)
- [ ] PPO + FedAvg on CartPole-v1 (simulated N-device fleet)
- [ ] Edge hardware fleet benchmark — Raspberry Pi fleet candidate
- [ ] Publish results in `results/fdrl/` with full extended methodology table
- [ ] External methodology review before arXiv submission
- [ ] Submit to arXiv

---

## Ongoing

- [ ] Keep dependencies on latest stable versions
- [ ] `cargo audit` clean at all times
- [ ] `cargo clippy -- -D warnings` clean at all times
- [ ] CHANGELOG.md maintained per [Keep a Changelog](https://keepachangelog.com/)
- [ ] Never publish a result without the full methodology table from ARCHITECTURE.md
- [ ] `results/` is append-only — published results are never overwritten
- [ ] All benchmark results include full methodology table from [ARCHITECTURE.md](ARCHITECTURE.md)
- [ ] All benchmark results include full hardware specification
