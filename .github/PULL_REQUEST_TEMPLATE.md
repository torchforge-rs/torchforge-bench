## Pull Request Checklist

### Description
<!-- Brief description of changes and motivation -->

### Linked Issue
<!-- Reference to related issue(s), e.g., "Fixes #123" -->

### Changes Made
<!-- List of major changes -->

### Algorithm Changes
- [ ] **Re-benchmarked**: If touching algorithm implementations, affected benchmarks have been re-run
- [ ] **Results Updated**: `results/` directory updated with new benchmark results

### Methodology
- [ ] **Methodology Table**: Complete methodology table populated for any benchmark changes
- [ ] **Results File**: JSON results file committed to `results/` directory
- [ ] **Hardware Specs**: Hardware specifications documented
- [ ] **Software Versions**: All software versions documented
- [ ] **Random Seeds**: Seeds used for reproducibility documented

### Documentation
- [ ] **CHANGELOG Entry**: Added entry under appropriate section
- [ ] **API Docs**: Updated API documentation if needed
- [ ] **README**: Updated README if needed

### Testing
- [ ] **Unit Tests**: All unit tests pass
- [ ] **Integration Tests**: All integration tests pass
-- [ ] **Manual Testing**: Changes manually tested
- [ ] **Benchmark Reproducibility**: Results can be reproduced

### Code Quality
- [ ] **Formatted**: Code formatted with `cargo fmt`
- [ ] **Linted**: Code passes `cargo clippy -- -D warnings`
- [ ] **Documentation**: Public APIs documented

## Additional Information

<!-- Any additional information reviewers should know -->

## Review Focus Areas

<!-- Specific areas you'd like reviewers to focus on -->

---

### Before Submitting

1. **Run Tests**: Ensure all tests pass locally
2. **Format Code**: Run `cargo fmt`
3. **Lint Code**: Run `cargo clippy -- -D warnings`
4. **Check Documentation**: Ensure docs are updated
5. **Verify Benchmarks**: Re-run affected benchmarks if applicable

### Merge Requirements

- [ ] All automated checks pass
- [ ] At least one review approved
- [ ] All checklist items completed
- [ ] No merge conflicts
