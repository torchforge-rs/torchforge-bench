# Changelog

All notable changes to TorchForge Bench will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Initial project foundation and repository structure
- Apache-2.0 license and governance documents
- GitHub templates for issues and pull requests
- Python baseline infrastructure with CleanRL integration
- Results directory structure and JSON schema
- Comprehensive documentation and contribution guidelines

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.0.1] - 2024-03-26

### Added
- Initial repository setup with Rust project structure
- Core project configuration (Cargo.toml, rust-toolchain.toml)
- Expanded .gitignore for comprehensive development coverage
- Documentation foundation (README.md, ARCHITECTURE.md, TODO.md)

---

## Version Policy

### v0.x - Pre-Alpha/Alpha Phase
- No stability guarantees
- Breaking changes may occur without notice
- API subject to significant changes
- Results schema may evolve
- Use for experimentation and feedback only

### v1.x - Stable Release
- Stability guarantees
- Backward compatibility maintained within major version
- API stability ensured
- Results schema stable
- Suitable for production use

### v2.x and beyond
- Major feature additions
- Potential breaking changes
- New algorithm support
- Performance improvements

---

## Change Types

### Added
- New features
- New algorithms
- New environments
- New documentation
- Benchmark results

### Changed
- Modifications to existing functionality
- Updated documentation
- Parameter changes
- Performance improvements

### Deprecated
- Features marked for future removal
- API changes planned for next major version
- Migration notices

### Removed
- Deleted features
- API removals
- Algorithm deprecations completed

### Fixed
- Bug fixes
- Performance issues
- Documentation corrections
- Result inaccuracies

### Security
- Security vulnerability fixes
- Dependency updates
- Security policy changes
- Access control improvements

---

## Benchmark Results Policy

All benchmark result updates are documented under the **Added** section, including:

- New algorithm implementations
- Updated hardware specifications
- Methodology improvements
- Reproducibility enhancements
- Performance optimizations

Each result entry includes:
- Algorithm and environment
- Hardware/software specifications
- Performance metrics
- Reproducibility information
- Comparison to previous results

---

## Contributing to Changelog

When contributing to the project:

1. **Add Entry**: Every PR requires a CHANGELOG entry
2. **Categorize**: Use appropriate section (Added, Changed, Fixed, etc.)
3. **Describe**: Clear, concise description of changes
4. **Reference**: Link to relevant issues or PRs
5. **Date**: Include release date for new versions

### Entry Format

```markdown
### Added
- [Feature description] ([#123](https://github.com/torchforge-rs/torchforge-bench/issues/123))
- [Benchmark result: DQN CartPole on RTX 4070] ([#124](https://github.com/torchforge-rs/torchforge-bench/issues/124))

### Fixed
- [Bug description] ([#125](https://github.com/torchforge-rs/torchforge-bench/issues/125))
```

---

## Release Process

1. **Update Version**: Bump version in Cargo.toml
2. **Update Changelog**: Move unreleased changes to version section
3. **Add Date**: Include release date
4. **Create Tag**: Create Git tag for release
5. **Publish**: Publish to crates.io (when applicable)
6. **Announce**: Announce release in discussions

---

## Historical Context

This changelog begins with the initial project setup on 2024-03-26. All subsequent development will be documented here to provide a complete history of the project's evolution.

For detailed development history between releases, refer to the Git commit log and individual pull request descriptions.
