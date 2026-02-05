# Security Policy

DK-AppStore is critical national infrastructure. We take security seriously.

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| main    | :white_check_mark: |
| develop | :white_check_mark: |
| < 1.0   | Phase 0 - Pre-release |

## Reporting a Vulnerability

### Do NOT

- Open a public issue for security vulnerabilities
- Discuss vulnerabilities in public channels
- Exploit vulnerabilities beyond proof-of-concept

### Do

1. **Email**: TBD
2. **Encrypt** your report using our PGP key (below)
3. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact assessment
   - Suggested fix (if any)

### PGP Key

```
-----BEGIN PGP PUBLIC KEY BLOCK-----
[Key to be published upon project launch]
-----END PGP PUBLIC KEY BLOCK-----
```

Key fingerprint: `[To be published]`

### What to Expect

| Timeframe | Action |
|-----------|--------|
| 24 hours  | Acknowledgment of report |
| 72 hours  | Initial assessment |
| 7 days    | Status update |
| 90 days   | Target resolution (critical: 30 days) |

### Safe Harbor

We consider security research conducted in good faith to be authorized. We will not pursue legal action against researchers who:

- Make a good faith effort to avoid privacy violations
- Avoid destruction of data
- Do not exploit beyond proof-of-concept
- Report vulnerabilities promptly and privately

## Security Measures

### Code Security

- **Memory Safety**: Rust as primary language eliminates buffer overflows, use-after-free
- **No Unsafe Code**: `unsafe` blocks forbidden without explicit approval
- **Dependency Auditing**: `cargo audit` and `cargo deny` in CI
- **Static Analysis**: `cargo clippy` with strict settings
- **Secret Detection**: Gitleaks in pre-commit and CI

### Cryptographic Security

- **HSM-Based Signing**: Keys never leave hardware security modules
- **Audited Libraries**: Using `ring` and `rustls` (BoringSSL-derived)
- **No Custom Crypto**: Standard algorithms only (Ed25519, AES-256-GCM)
- **Key Ceremony**: Multi-party procedures for key generation

### Infrastructure Security

- **Air-Gapped Signing**: Signing servers physically isolated
- **Reproducible Builds**: Verified by multiple independent builders
- **Minimal Attack Surface**: Single static binary deployments
- **No Root**: Rootless containers (Podman)

### Supply Chain Security

- **SBOM Generation**: Full Software Bill of Materials for all releases
- **Dependency Pinning**: Exact versions in Cargo.lock
- **License Compliance**: `cargo deny` checks licenses
- **Signed Releases**: All releases cryptographically signed

## Security Checklist for Contributors

Before submitting code:

- [ ] No hardcoded secrets, keys, or credentials
- [ ] No `unsafe` blocks (or justified and approved)
- [ ] No `.unwrap()` or `.expect()` in error paths
- [ ] Input validation on all external data
- [ ] Proper error handling (no sensitive data in errors)
- [ ] `cargo audit` passes
- [ ] `cargo deny check` passes

### Sensitive Areas

Extra review required for changes to:

| Path | Sensitivity | Reviewers Required |
|------|-------------|-------------------|
| `dk-signing/` | Critical | 2 + Security Team |
| `dk-build/src/verify.rs` | Critical | 2 + Security Team |
| `dk-api/src/routes/admin/` | High | 2 |
| `**/auth*.rs` | High | 2 |
| `Cargo.toml` (dependencies) | Medium | 1 + cargo audit |

## Incident Response

### Severity Levels

| Level | Description | Response Time |
|-------|-------------|---------------|
| **Critical** | Active exploitation, key compromise | 15 minutes |
| **High** | Exploitable vulnerability, no active exploitation | 1 hour |
| **Medium** | Vulnerability requiring specific conditions | 24 hours |
| **Low** | Minor issue, limited impact | 7 days |

### Response Process

1. **Triage**: Assess severity and impact
2. **Containment**: Isolate affected systems if needed
3. **Investigation**: Determine root cause
4. **Remediation**: Develop and test fix
5. **Communication**: Notify affected parties
6. **Post-Incident**: Document lessons learned

### Emergency Contacts

- **Security Team**: security@digst.dk
- **On-Call (Critical)**: [To be established in Phase 1]
- **CFCS Coordination**: [Contact TBD]

## Compliance

DK-AppStore security practices align with:

- **ISO 27001** - Information Security Management
- **NSIS** - Danish National Standard for Identity Security
- **NIS2** - EU Network and Information Security Directive
- **Common Criteria** - EAL4+ for critical components

## Security Advisories

Published security advisories: [To be linked upon release]

Subscribe to security notifications: [To be established]

## Acknowledgments

We thank the security researchers who help keep DK-AppStore secure:

| Researcher | Finding | Date |
|------------|---------|------|
| *None yet* | - | - |

---

*Last updated: 2026-02-04*
