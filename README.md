# torchforge-bench

> Reproducible ML benchmark suite in Rust — proving edge-native training is viable, one algorithm at a time.

Part of the [torchforge-rs](https://github.com/torchforge-rs) ecosystem.

[![Crates.io](https://img.shields.io/crates/v/torchforge-bench.svg)](https://crates.io/crates/torchforge-bench)
[![Docs.rs](https://docs.rs/torchforge-bench/badge.svg)](https://docs.rs/torchforge-bench)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![CI](https://github.com/torchforge-rs/torchforge-bench/actions/workflows/ci.yml/badge.svg)](https://github.com/torchforge-rs/torchforge-bench/actions/workflows/ci.yml)

---

## Why

Two facts are simultaneously true today:

1. No published benchmark demonstrates a high-profile ML algorithm reproduced in Rust outperforming its Python reference implementation
2. The absence of such a benchmark is the primary reason practitioners do not take Rust ML seriously

`torchforge-bench` exists to close this gap — not with claims, but with reproducible numbers on documented hardware.

The reference target is [CleanRL](https://github.com/vwxyzjn/cleanrl) — the most reproducible Python RL benchmark suite, with single-file implementations and published results. We reproduce CleanRL algorithms in Rust, measure against the same environments on the same hardware, and publish everything, including results where Rust is slower.

---

## Design Principles

1. **Reproducibility over performance** — a benchmark that cannot be reproduced is worthless
2. **Documented hardware** — every published result names the exact machine it was run on
3. **Apples-to-apples** — same environment, same hyperparameters, same metric definitions as the Python reference
4. **No cherry-picking** — we benchmark what we build, not what makes us look good
5. **Honest about unknowns** — if a result is surprising, we investigate before claiming

---

## Status

**`v0.0.1` — Pre-alpha. No benchmarks published yet. Three prerequisite research items block v0.1.0.**

The repository structure, CI, governance documents, baseline infrastructure (`baselines/` via `uv`), and results schema are complete. Before algorithm implementation begins:

1. **`[RESEARCH]`** — PyO3 FFI overhead on `env.step()` must be measured and documented
2. **`[RESEARCH]`** — Neural network backend prototype (`burn`+`ndarray` vs `candle`) must be completed
3. **`[RESEARCH]`** — CleanRL DQN Python baseline must be run on target hardware and results stored

See [ARCHITECTURE.md](ARCHITECTURE.md) for rationale and [TODO.md](TODO.md) for the full roadmap.

---

## Roadmap

| Version | Goal |
|---|---|
| **Pre-v0.1.0** | FFI overhead measurement, NN backend prototype, Python baseline (hard blockers) |
| **v0.1.0** | DQN on CartPole-v1 — results published against CleanRL, including if slower |
| **v0.2.0** | PPO on CartPole-v1 |
| **v0.3.0** | Edge hardware benchmarks (ARM, Raspberry Pi 5 candidate) |
| **v0.4.0** | SAC on continuous control |
| **v1.0.0** | First stable benchmark, externally reviewed methodology |
| **v1.x** | First reproducible FDRL benchmark suite in Rust for edge hardware — arXiv target |

The v1.x FDRL benchmark is the north star. Every methodology decision made at v0.x is made with the requirement that it holds under the federated case: same hardware documentation, same seed discipline, same publication standards — applied across a fleet of devices rather than one.

---

## Algorithm Targets

| Algorithm | Reference | Environment | Status |
|---|---|---|---|
| DQN | CleanRL `dqn.py` | CartPole-v1 | 🔲 Pre-requisite research |
| PPO | CleanRL `ppo.py` | CartPole-v1 | 🔲 Blocked on v0.1.0 |
| SAC | CleanRL `sac_continuous_action.py` | TBD | 🔲 Blocked on v0.3.0 |

---

## Reproducing Baselines

Python baselines are managed via [`uv`](https://docs.astral.sh/uv/) with a committed lockfile — reproducible installs, no loose pip.

```bash
# Install uv if needed
curl -LsSf https://astral.sh/uv/install.sh | sh

# Run the DQN CartPole baseline
cd baselines/
uv run python dqn_cartpole.py

# Results are written to ../results/baselines/
```

Rust benchmarks run via the standard `cargo bench` interface once v0.1.0 is implemented:

```bash
cargo bench --bench dqn_cartpole
```

Every published result includes: exact hardware, OS, Rust version, Python/PyTorch version, seeds (minimum 5, mean ± std), wall-clock time (total and training-only), and peak memory. No result is published without the full methodology table. See [ARCHITECTURE.md](ARCHITECTURE.md) for the complete specification.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide — prerequisites (Rust, Python, `uv`), branching model, PR process, and the result reproducibility policy.

The most valuable contributions right now are:

- Running the CleanRL DQN baseline on your hardware and documenting results — this directly unblocks v0.1.0
- Running the NN backend prototypes (`burn`+`ndarray` vs `candle`) and reporting compile time, binary size, and autodiff correctness
- Measuring PyO3 FFI overhead on `CartPole-v1 env.step()`
- Challenging assumptions in [ARCHITECTURE.md](ARCHITECTURE.md)

**Open an issue before submitting a PR.**

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before participating.
Benchmark result disputes are handled via the `methodology_challenge` issue template, not as security issues — see [SECURITY.md](SECURITY.md) for what does qualify.

---

## License

Apache-2.0. See [LICENSE](LICENSE).
CleanRL baseline scripts in `baselines/` are MIT licensed — see `baselines/README.md` for attribution.

Part of the [torchforge-rs](https://github.com/torchforge-rs) ecosystem — also see [torchforge-data](https://github.com/torchforge-rs/torchforge-data) and [torchforge-viz](https://github.com/torchforge-rs/torchforge-viz).
