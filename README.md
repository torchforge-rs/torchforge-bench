# torchforge-bench

> Reproducible ML benchmark suite in Rust — proving edge-native training is viable, one algorithm at a time.

Part of the [torchforge-rs](https://github.com/torchforge-rs) ecosystem.

[![Crates.io](https://img.shields.io/crates/v/torchforge-bench.svg)](https://crates.io/crates/torchforge-bench)
[![Docs.rs](https://docs.rs/torchforge-bench/badge.svg)](https://docs.rs/torchforge-bench)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)

---

## Why

Two facts are simultaneously true today:

1. No published benchmark demonstrates a high-profile ML algorithm reproduced in Rust outperforming its Python reference implementation
2. The absence of such a benchmark is the primary reason practitioners do not take Rust ML seriously

`torchforge-bench` exists to close this gap — not with claims, but with reproducible numbers on documented hardware.

The reference target is [CleanRL](https://github.com/vwxyzjn/cleanrl) — the most reproducible Python RL benchmark suite, with single-file implementations and published results. We reproduce CleanRL algorithms in Rust, measure against the same environments on the same hardware, and publish everything.

If we are slower, we say so. If we are faster, we prove it.

---

## Design Principles

1. **Reproducibility over performance** — a benchmark that cannot be reproduced is worthless
2. **Documented hardware** — every published result names the exact machine it was run on
3. **Apples-to-apples** — same environment, same hyperparameters, same metric definitions as the Python reference
4. **No cherry-picking** — we benchmark what we build, not what makes us look good
5. **Honest about unknowns** — if a result is surprising, we investigate before claiming

---

## Status

`v0.0.x` — **Pre-alpha. No benchmarks published yet. Active design phase.**

See [ARCHITECTURE.md](ARCHITECTURE.md) for methodology and open questions.
See [TODO.md](TODO.md) for the implementation roadmap.

---

## Target Algorithms

In priority order (subject to change):

| Algorithm | Reference | Status |
|---|---|---|
| DQN | CleanRL `dqn.py` | 🔲 Not started |
| PPO | CleanRL `ppo.py` | 🔲 Not started |
| SAC | CleanRL `sac_continuous_action.py` | 🔲 Not started |

---

## Contributing

The most valuable contributions right now are:

- Running existing Python CleanRL baselines and documenting results on your hardware
- Identifying environment drivers that work from Rust without Python FFI
- Challenging assumptions in [ARCHITECTURE.md](ARCHITECTURE.md)

Open an issue before submitting a PR.

---

## License

Apache-2.0. See [LICENSE](LICENSE).
