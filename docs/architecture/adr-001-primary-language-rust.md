# ADR-001: Rust as Primary Implementation Language

**Status:** Accepted  
**Date:** 2026-02-04  
**Deciders:** DIGST Technical Team  
**Context:** Technology selection for DK-AppStore server components

---

## Context

DK-AppStore is a sovereign Danish app distribution platform designed to host critical national applications including MitID (national digital identity). The platform handles:

- Cryptographic signing of applications
- Verification of reproducible builds
- Storage and distribution of applications to millions of Danish citizens
- Security scanning and threat detection

The choice of primary implementation language has significant implications for:
1. **Security posture** - The platform handles cryptographic keys and determines application trustworthiness
2. **Performance** - Must handle high concurrent loads during peak usage
3. **Long-term maintainability** - Platform will operate for 10+ years
4. **Audit confidence** - Code will undergo security certification (Common Criteria, ISO 27001)

---

## Decision

**Rust is selected as the primary implementation language** for all new DK-AppStore server components, including:

- Repository API server
- Build verification workers
- Signing service
- Security scanning orchestration

Python will be used only for interfacing with fdroidserver where direct integration is required.

---

## Rationale

### Why Rust

| Factor | Benefit |
|--------|---------|
| **Memory safety without garbage collection** | Eliminates buffer overflows, use-after-free, and null pointer dereferences at compile time. Critical for signing service handling cryptographic keys. |
| **Thread safety guarantees** | Compiler enforces safe concurrency via ownership system. Prevents data races in parallel build verification. |
| **Performance** | Near-C performance with zero-cost abstractions. Handles high concurrent download loads efficiently. |
| **Strong type system** | Errors caught at compile time. Reduces runtime failures in production. |
| **Excellent cryptography ecosystem** | `ring`, `rustls`, `pkcs11` crates are well-audited and production-proven. |
| **No runtime dependencies** | Single static binary deployment. Minimal attack surface in containers. |
| **Formal verification support** | Tools like Kani and Creusot enable mathematical proofs of correctness for critical code paths. |
| **Government endorsement** | NSA, CISA, and NCSC recommend Rust for security-critical systems. Used in Android, Linux kernel, Windows. |
| **Audit confidence** | Memory safety guarantees reduce scope of security audits. Auditors can focus on logic rather than memory bugs. |

### Security Argument

The signing service and build verification components are the most security-critical parts of DK-AppStore:

- **Signing service**: Handles HSM communication and cryptographic keys. A memory corruption vulnerability could leak keys or allow unauthorized signing.
- **Build verification**: Compares hashes from multiple builders. A bug could allow mismatched builds to pass verification.

In Rust, entire categories of vulnerabilities are impossible:

| Vulnerability Class | C/C++ | Python | Go | Rust |
|---------------------|-------|--------|-----|------|
| Buffer overflow | Possible | Rare | Impossible | Impossible |
| Use-after-free | Possible | Impossible (GC) | Impossible (GC) | Impossible |
| Null pointer dereference | Possible | Possible | Possible (nil) | Impossible (Option type) |
| Data races | Possible | Possible | Possible | Impossible |
| Integer overflow | Undefined | Handled | Wraps | Configurable (panic/wrap) |

For national identity infrastructure serving 6 million citizens, eliminating these vulnerability classes is worth the development investment.

---

## Alternatives Considered

### Python with FastAPI

**Initial consideration:** fdroidserver is written in Python, suggesting a Python-based API wrapper.

**Reasons for rejection:**

| Concern | Detail |
|---------|--------|
| **No memory safety guarantees** | Runtime type checking only. Type hints are not enforced. |
| **Performance limitations** | Global Interpreter Lock (GIL) prevents true parallelism. Async helps I/O-bound work but not CPU-bound verification. |
| **Runtime errors** | `None` handling, type mismatches, and attribute errors discovered only at runtime. |
| **Security audit burden** | Auditors must verify absence of type confusion, injection, and deserialization vulnerabilities manually. |
| **Dependency risks** | Large transitive dependency trees. PyPI packages not consistently audited. |
| **Deployment complexity** | Requires Python runtime, virtual environments, and dependency management in containers. |

**Specific concerns for DK-AppStore:**

1. **Signing service in Python**: Handling PKCS#11 HSM communication in Python relies on ctypes FFI bindings. Memory management errors in FFI code can cause crashes or security vulnerabilities.

2. **Build verification**: Hash comparison and binary diffing are CPU-intensive. Python's GIL would serialize this work, limiting throughput.

3. **Security certification**: Auditors familiar with memory-safe languages will require extensive additional testing for Python components.

**Verdict:** Python is acceptable for scripting and fdroidserver integration but unsuitable for security-critical server components.

### Go

**Consideration:** Go offers simplicity, good concurrency, and fast compilation. Used successfully by Docker, Kubernetes, and similar infrastructure projects.

**Reasons for rejection:**

| Concern | Detail |
|---------|--------|
| **Garbage collection pauses** | GC can introduce latency spikes. Less predictable than Rust for real-time signing operations. |
| **No compile-time thread safety** | Data races possible despite goroutines. Detected only by race detector at runtime. |
| **Nil pointer panics** | `nil` interface and pointer values can cause runtime panics. No compile-time null safety. |
| **Less expressive type system** | No sum types (until recently), no generics (until Go 1.18), limited compile-time guarantees. |
| **Error handling verbosity** | `if err != nil` patterns obscure control flow. Errors can be silently ignored. |
| **Weaker cryptography story** | Standard library crypto is good but `ring` (Rust) has stronger audit pedigree. PKCS#11 bindings less mature. |
| **Formal verification** | Limited tooling compared to Rust ecosystem. |

**Where Go would be acceptable:**

- Non-security-critical tooling (CLI utilities, log processors)
- Rapid prototyping before Rust implementation
- Components where team expertise strongly favors Go

**Verdict:** Go is a reasonable choice for general infrastructure but does not provide the safety guarantees required for signing and verification components.

---

## Consequences

### Positive

1. **Stronger security posture** - Memory safety eliminates entire vulnerability classes
2. **Audit efficiency** - Security reviewers can focus on business logic rather than memory bugs
3. **Performance headroom** - No GC pauses, efficient concurrency for high load
4. **Long-term maintainability** - Compiler catches errors during refactoring
5. **Talent attraction** - Security-focused developers increasingly prefer Rust
6. **Future-proofing** - Growing adoption in security-critical infrastructure (Linux kernel, Android, AWS)

### Negative

1. **Steeper learning curve** - Team members unfamiliar with Rust will need training (estimate: 4-6 weeks to productivity)
2. **Slower initial development** - Compiler strictness requires more upfront design (offset by fewer production bugs)
3. **Smaller talent pool** - Fewer Rust developers than Python/Go (mitigated by growing community and Nordic presence)
4. **fdroidserver integration** - Requires subprocess calls or Python FFI rather than direct integration
5. **Compile times** - Longer than Go (mitigated by incremental compilation and `cargo check`)

### Mitigations

| Challenge | Mitigation |
|-----------|------------|
| Learning curve | Allocate training time. Pair experienced Rust developers with team. Use "Rust for Rustaceans" and official book. |
| Initial velocity | Accept slower Phase 0 velocity. Emphasize correctness over speed. |
| Talent pool | Partner with Nordic Rust community. Sponsor RustFest/EuroRust. Competitive compensation. |
| fdroidserver integration | Isolate Python integration layer. Call fdroidserver via well-defined subprocess interface. |
| Compile times | Use `cargo check` during development. Optimize CI caching. Consider `sccache`. |

---

## Implementation Notes

### Recommended Rust Stack

| Component | Crate | Rationale |
|-----------|-------|-----------|
| **Web framework** | `axum` | Tokio-based, tower middleware, type-safe extractors |
| **Async runtime** | `tokio` | Industry standard, mature, well-documented |
| **Database** | `sqlx` | Compile-time query verification, async, PostgreSQL native |
| **Serialization** | `serde` | De facto standard, zero-copy deserialization |
| **HTTP client** | `reqwest` | Built on hyper, TLS via rustls |
| **Cryptography** | `ring`, `rustls` | BoringSSL-derived, audited, no OpenSSL dependency |
| **HSM/PKCS#11** | `cryptoki` | Pure Rust PKCS#11 bindings |
| **CLI** | `clap` | Derive macros, shell completions |
| **Logging** | `tracing` | Structured logging, async-aware, OpenTelemetry integration |
| **Testing** | `cargo test` + `proptest` | Built-in + property-based testing |
| **Error handling** | `thiserror`, `anyhow` | Ergonomic error types |

### Minimum Supported Rust Version (MSRV)

- **MSRV:** Rust 1.75 (stable, released December 2024)
- **Policy:** Support current stable minus two versions
- **Rationale:** Balance between new features and distribution package availability

### Code Quality Standards

```toml
# Cargo.toml excerpt
[lints.rust]
unsafe_code = "forbid"  # No unsafe in application code
missing_docs = "warn"

[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
cargo = "warn"
```

---

## References

- [NSA Cybersecurity Information Sheet: Software Memory Safety](https://media.defense.gov/2022/Nov/10/2003112742/-1/-1/0/CSI_SOFTWARE_MEMORY_SAFETY.PDF)
- [CISA: The Case for Memory Safe Roadmaps](https://www.cisa.gov/resources-tools/resources/case-memory-safe-roadmaps)
- [Rust in the Linux Kernel](https://rust-for-linux.com/)
- [Android Rust Support](https://source.android.com/docs/setup/build/rust/building-rust-modules/overview)
- [AWS: Why AWS loves Rust](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help/)

---

## Decision Record

| Date | Action | Author |
|------|--------|--------|
| 2026-02-04 | Initial decision documented | DIGST Technical Team |

