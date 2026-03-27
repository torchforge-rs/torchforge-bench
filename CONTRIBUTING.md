# Contributing to TorchForge Bench

Thank you for your interest in contributing to TorchForge Bench! This document provides guidelines for contributors.

## Prerequisites

### System Requirements
- **Rust**: 1.85.0 or higher (use `rust-toolchain.toml` for automatic version management)
- **Python**: 3.12 or higher
- **uv**: Python package manager for baseline scripts

### Dependencies

#### Rust Dependencies
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Format and lint tools
rustup component add rustfmt
rustup component add clippy
```

#### Python Dependencies
```bash
# Install uv (if not already installed)
curl -LsSf https://astral.sh/uv/install.sh | sh

# Install baseline dependencies
uv sync --cwd baselines
```

#### Gymnasium Dependencies
```bash
# Install Gymnasium with classic control environments
pip install gymnasium[classic-control]
```

## Development Workflow

### Building and Testing

#### Rust Code
```bash
# Build the project
cargo build

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
```

#### Python Baselines
```bash
# Run baseline script
uv run python baselines/dqn_cartpole.py

# Install new Python dependencies
uv add <package> --cwd baselines
```

### Branching Model

1. **Main Branch**: `main` - Always stable, released versions only
2. **Development Branches**: `feature/<description>` - For new features
3. **Bugfix Branches**: `bugfix/<description>` - For bug fixes
4. **Hotfix Branches**: `hotfix/<description>` - For urgent fixes to main

### Pull Request Process

1. **Fork** the repository
2. **Create** a feature branch from `main`
3. **Make** your changes
4. **Test** thoroughly
5. **Submit** a pull request to `main`

#### Pull Request Requirements

Every pull request must include:

- [ ] **Description**: Clear explanation of changes and motivation
- [ ] **Linked Issue**: Reference to any related issues
- [ ] **Algorithm Changes**: If touching algorithm implementations, re-run affected benchmarks and update `results/`
- [ ] **Methodology Table**: populated for any benchmark result changes
- [ ] **Results File**: Updated JSON file in `results/` directory
- [ ] **CHANGELOG Entry**: Add entry under appropriate section

### Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `bench`: Benchmark changes
- `chore`: Maintenance tasks

#### Examples
```
feat(algorithms): add DQN implementation for CartPole

fix(baselines): resolve numpy version conflict in Python scripts

bench(results): update CartPole benchmark with new hardware specs
```

## Code Style

### Rust
- Use `cargo fmt` for formatting
- Use `cargo clippy -- -D warnings` for linting
- Follow Rust API guidelines
- Document public APIs with `///` doc comments
- Write unit tests for all public functions

### Python
- Follow PEP 8 style guide
- Use type hints where appropriate
- Document functions with docstrings
- Keep baseline scripts minimal and reproducible

## Testing

### Rust Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run ignored tests (e.g., statistical convergence tests)
cargo test -- --ignored

# Run tests with specific features
cargo test --features <feature_name>
```

### Python Baselines
```bash
# Run baseline with deterministic seed
uv run python baselines/dqn_cartpole.py --seed 42

# Run baseline with custom parameters
uv run python baselines/dqn_cartpole.py --env CartPole-v1 --episodes 1000
```

## Benchmark Results

### "Ready to Merge" Criteria for Benchmark Results

A benchmark result is considered ready to merge when:

1. **Methodology Table Complete**: All fields populated (hardware, OS, versions, seeds, metrics, wall-clock, memory)
2. **Hygiene Checklist Completed**: 
   - Results are reproducible
   - Hardware specifications documented
   - Software versions documented
   - Random seeds documented
   - Environmental factors noted
3. **Results File Committed**: JSON file committed to `results/` directory following schema
4. **CHANGELOG Entry**: Added under `[Added]` section

### Result Reproducibility Policy

- **Algorithm Changes**: Any PR touching algorithm implementations must re-run affected benchmarks
- **Environment Changes**: Any changes to build system or dependencies must update results
- **Hardware Documentation**: All results must include complete hardware specifications
- **Version Tracking**: All software versions must be documented in results

## Issue Templates

### Bug Reports
When filing bug reports, please include:
- Rust version (`rustc --version`)
- Python version (`python --version`)
- Operating system and version
- Hardware specifications
- Complete reproduction steps
- Expected vs actual behavior
- Any error messages or logs

### Methodology Challenges
For disputing published results:
- What specific result is incorrect
- Proposed correction with supporting evidence
- Reproduction steps demonstrating the issue
- References to supporting literature or data

### Feature Requests
For algorithm or environment requests:
- Clear description of the requested feature
- Rationale for inclusion in the benchmark suite
- Any relevant literature or references
- Implementation suggestions (if any)

## Review Process

### Code Review
- All changes require at least one review
- Reviews focus on correctness, style, and documentation
- Automated checks must pass before merge

### Benchmark Review
- Benchmark results receive additional scrutiny
- Reviewers verify methodology completeness
- Results may be requested to be reproduced by reviewers

## Getting Help

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For general questions and discussion
- **Security Issues**: See `SECURITY.md` for reporting security vulnerabilities

## License

By contributing to this project, you agree that your contributions will be licensed under the same license as the project (Apache-2.0).

## Thank You

Thank you for contributing to TorchForge Bench! Your contributions help make ML benchmarking more reproducible and accessible.
