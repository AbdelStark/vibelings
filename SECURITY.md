# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Model

vibelings implements a defense-in-depth security model:

### Sandbox Security

- **Command allowlisting**: Only explicitly permitted commands can execute
- **Network isolation**: Network access disabled by default
- **Filesystem confinement**: Tool execution confined to exercise workspace
- **Timeout enforcement**: All tool executions have configurable timeouts
- **Trace auditing**: All tool calls are logged for review

### API Key Security

- API keys are **never** stored in configuration files directly
- Keys are read from environment variables at runtime
- Zero Data Retention (ZDR) is enabled by default on OpenRouter
- No keys or credentials are transmitted to exercise graders

### Exercise Security

- Exercise content is sandboxed and cannot access system resources
- Grading is deterministic and does not require LLM evaluation of untrusted content
- All exercises are reviewed for security before inclusion

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please report it responsibly.

### How to Report

1. **DO NOT** create a public GitHub issue for security vulnerabilities
2. Email your report to the repository maintainers (see GitHub profile)
3. Or use GitHub's [private vulnerability reporting](https://github.com/AbdelStark/vibelings/security/advisories/new) feature

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact assessment
- Any suggested fixes (optional)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Resolution Target**: Within 30 days for critical issues

### What to Expect

1. We will acknowledge receipt of your report
2. We will investigate and assess the severity
3. We will work on a fix if the issue is confirmed
4. We will coordinate disclosure timing with you
5. We will credit you in the security advisory (unless you prefer to remain anonymous)

## Security Best Practices for Users

### API Key Management

```bash
# Good: Use environment variables
export OPENROUTER_API_KEY="your-key-here"

# Bad: Don't put keys in config files or command line
# vibelings --api-key "..." # NEVER DO THIS
```

### Sandbox Configuration

```toml
# Recommended security settings
[sandbox]
network = false          # Disable network access
timeout_seconds = 30     # Limit execution time
allowed_commands = ["cat", "ls", "grep", "jq"]  # Minimal command set
```

### Exercise Review

Before running third-party exercises:

1. Review the `manifest.toml` for unusual requirements
2. Check for `network = true` which enables network access
3. Use `vibelings run --dry-run` to preview what will execute
4. Keep your sandbox configuration restrictive

## Security Boundaries

### In Scope

- Sandbox escape vulnerabilities
- Command injection via exercise content
- API key leakage
- Unauthorized network access
- File system access outside workspace
- Code execution in grading scripts

### Out of Scope

- Vulnerabilities in upstream dependencies (report to those projects)
- Model provider API security (report to OpenRouter, OpenAI, etc.)
- Social engineering attacks
- Physical access attacks
- Denial of service (resource exhaustion)

## Acknowledgments

We appreciate the security research community's efforts in helping keep vibelings secure. Contributors who report valid vulnerabilities will be acknowledged here (with permission).

---

*This security policy follows responsible disclosure best practices.*
