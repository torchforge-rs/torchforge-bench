# Security Policy

## Supported Versions

Only the latest `v0.x` versions are supported with security updates.

| Version | Supported |
|---------|-----------|
| latest v0.x | ✅ |
| v1.x and later | ✅ (when released) |
| older versions | ❌ |

## Reporting a Vulnerability

### Private Reporting

We encourage responsible disclosure of security vulnerabilities. Please report security issues privately via GitHub Security Advisories:

1. Visit the [Security Advisories page](https://github.com/torchforge-rs/torchforge-bench/security/advisories)
2. Click "Report a vulnerability"
3. Provide detailed information about the vulnerability

### What to Include

Please include as much information as possible in your report:

- **Vulnerability Type**: What kind of vulnerability it is
- **Affected Versions**: Which versions are affected
- **Impact**: Potential impact of the vulnerability
- **Reproduction Steps**: Step-by-step instructions to reproduce
- **Proof of Concept**: Code or examples demonstrating the vulnerability
- **Mitigation Suggestions**: Any suggested fixes or mitigations

### Response Timeline

- **Acknowledgment**: Within 72 hours
- **Triage**: Within 7 days
- **Resolution**: As appropriate based on severity
- **Public Disclosure**: After fix is released, with appropriate credit

## Scope

### In Scope

Security issues that are in scope for this policy include:

- **Soundness Issues**: Problems in `unsafe` blocks that could cause memory safety violations
- **Supply Chain Issues**: Vulnerabilities in dependencies that could affect the project
- **Data Access**: Unauthorized access to sensitive data
- **API Boundaries**: Bypassing security controls in public APIs
- **Code Execution**: Arbitrary code execution vulnerabilities

### Out of Scope

The following are NOT considered security issues:

- **Benchmark Result Disputes**: Disagreements about published benchmark results
  - These should be reported as regular issues using the "methodology challenge" template
- **Performance Issues**: Performance problems that don't pose security risks
- **Feature Requests**: Requests for new functionality
- **Documentation Issues**: Errors or omissions in documentation
- **Build Issues**: Problems building the project on unsupported platforms

## Security Best Practices

### For Users

- **Keep Updated**: Use the latest supported version
- **Review Dependencies**: Regularly update dependencies
- **Isolate Environments**: Run benchmarks in isolated environments
- **Verify Results**: Independently verify critical benchmark results

### For Contributors

- **Safe Code**: Follow Rust safety guidelines, minimize `unsafe` usage
- **Dependency Hygiene**: Keep dependencies updated and reviewed
- **Input Validation**: Validate all external inputs
- **Error Handling**: Handle errors securely without leaking information

## Security Team

The security team reviews and responds to security reports. Team members include:

- Project maintainers
- Security advisors (as needed)
- External security researchers (as needed)

## Disclosure Policy

### Coordination

We coordinate with reporters to ensure responsible disclosure:

- **Private Fix**: Develop and test fixes before public disclosure
- **Coordinated Release**: Announce vulnerabilities when fixes are available
- **Credit**: Give credit to reporters who follow responsible disclosure

### Public Disclosure

Public disclosure includes:

- **Security Advisory**: Published on GitHub
- **CVE Assignment**: Requested when appropriate
- **Fix Release**: Included in next release
- **Documentation**: Updated documentation as needed

## Security Updates

### Update Process

Security updates are handled through:

- **Patch Releases**: For critical vulnerabilities
- **Minor Releases**: For less critical issues
- **Security Advisories**: Published with details and mitigations

### Notification

Users are notified of security updates through:

- **GitHub Releases**: Release notes include security information
- **Security Advisories**: Published on GitHub
- **Repository Announcements**: Important security announcements

## Security Testing

### Automated Testing

We use automated security testing:

- **Cargo Audit**: Checks for known vulnerabilities in dependencies
- **Cargo Deny**: Enforces dependency policies and license compliance
- **Static Analysis**: Automated code analysis for security issues

### Manual Review

Security-critical code receives manual review:

- **Unsafe Blocks**: All `unsafe` code is carefully reviewed
- **External Interfaces**: Public APIs are reviewed for security issues
- **Dependencies**: New dependencies are reviewed for security implications

## Contact

For security-related questions not fitting the vulnerability report format:

- **Email**: security@torchforge.org (if available)
- **GitHub**: Use the security advisory system for formal reports

## Acknowledgments

We thank security researchers who help us keep TorchForge Bench secure. All responsible disclosures will be acknowledged in security advisories and release notes.
