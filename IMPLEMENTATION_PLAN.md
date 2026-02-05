# DK-AppStore Implementation Plan

## Phase 0: Open Source Foundation

**Document Version:** 0.2  
**Status:** Draft  
**Last Updated:** 2026-02-04  
**Primary Language:** Rust (see [ADR-001](docs/architecture/adr-001-primary-language-rust.md))

---

## 1. Executive Summary

This document outlines the implementation plan for the open source foundation of DK-AppStore, covering the pre-contract development phase. The goal is to establish a working prototype that can be handed over to a commercial vendor for production operations.

**Timeline:** 6 months (2026-01 to 2026-06)  
**Objective:** Functional F-Droid-based repository with Danish security enhancements

---

## 2. Technology Stack Decisions

> **Architecture Decision:** Rust is the primary implementation language for all server components.  
> See [ADR-001: Primary Language - Rust](docs/architecture/adr-001-primary-language-rust.md) for full rationale.

### 2.1 Core Infrastructure

| Component | Technology | Version | Rationale |
|-----------|------------|---------|-----------|
| **Repository Server** | Custom Rust service | - | Memory safety, performance, security audit confidence |
| **fdroidserver Integration** | Python subprocess | 2.x | Leverage existing fdroidserver for index generation |
| **Primary Language** | Rust | 1.75+ | Memory safety, thread safety, performance (see ADR-001) |
| **Web Framework** | axum | 0.7+ | Tokio-based, type-safe, tower middleware ecosystem |
| **Async Runtime** | tokio | 1.x | Industry standard, mature, well-documented |
| **Database** | PostgreSQL | 16+ | ACID compliance, JSON support, proven reliability |
| **Database Driver** | sqlx | 0.7+ | Compile-time query verification, async, pure Rust |
| **Cache Layer** | Redis | 7+ | Session management, rate limiting, pub/sub |
| **Container Runtime** | Podman | 4+ | Rootless containers, no daemon, OCI-compliant |
| **Container Orchestration** | Kubernetes | 1.28+ | Production-grade orchestration (Phase 1+) |
| **CI/CD** | GitLab CI | Self-hosted | Air-gap capable, Danish data residency |

### 2.2 Rust Crate Stack

| Purpose | Crate | Rationale |
|---------|-------|-----------|
| **Serialization** | serde, serde_json | De facto standard, zero-copy deserialization |
| **HTTP Client** | reqwest | Built on hyper, rustls TLS |
| **Cryptography** | ring, rustls | BoringSSL-derived, audited, no OpenSSL dependency |
| **HSM/PKCS#11** | cryptoki | Pure Rust PKCS#11 bindings |
| **CLI** | clap | Derive macros, shell completions |
| **Logging** | tracing, tracing-subscriber | Structured logging, async-aware, OpenTelemetry |
| **Error Handling** | thiserror, anyhow | Ergonomic error types |
| **Testing** | cargo test, proptest | Built-in + property-based testing |
| **Date/Time** | chrono, time | Timezone-aware datetime handling |
| **UUID** | uuid | Standard UUID generation |

### 2.3 Build Infrastructure

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Android SDK** | Command-line tools | Reproducible, scriptable builds |
| **Build Tool** | Gradle | 8.x | Android standard, dependency verification |
| **JDK** | Eclipse Temurin | 17 LTS | Open source, reproducible builds |
| **Build Isolation** | Podman + systemd-nspawn | Ephemeral, rootless, reproducible |
| **Dependency Verification** | Gradle Witness + SLSA | Supply chain integrity |
| **Build Orchestration** | Rust worker service | Type-safe job processing, safe concurrency |

### 2.4 Security Components

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Static Analysis (APK)** | MobSF | Comprehensive Android scanning |
| **Static Analysis (Rust)** | clippy, cargo-audit, cargo-deny | Lint, CVE detection, license compliance |
| **Dependency Scanning** | OWASP Dependency-Check, cargo-audit | CVE detection |
| **Secret Detection** | Gitleaks, truffleHog | Pre-commit and CI scanning |
| **SBOM Generation** | Syft, CycloneDX, cargo-sbom | Standard SBOM formats |
| **Signing (Dev)** | SoftHSM2 | HSM simulation for development |
| **Signing (Prod)** | Thales Luna / Utimaco | FIPS 140-3 Level 3 (Phase 1+) |

### 2.5 Client Applications

> **Architecture Decision:** Shared Rust core with platform-specific UI layers.  
> See [ADR-002: iOS Distribution Strategy](docs/architecture/adr-002-ios-distribution-strategy.md) for full rationale.

#### 2.5.1 Shared Client Core (Rust)

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Core Library** | `dk-client-core` crate | Single implementation of security-critical logic |
| **FFI Bindings** | UniFFI (iOS), JNI (Android) | Type-safe cross-language bindings |
| **Signature Verification** | ring | Consistent crypto across platforms |
| **Index Parsing** | serde | Shared serialization logic |
| **Certificate Pinning** | rustls | Memory-safe TLS with pinning |

#### 2.5.2 Android Client

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Base** | F-Droid Client fork | Proven codebase, active upstream |
| **Language** | Kotlin | Modern Android standard |
| **Core Integration** | JNI to dk-client-core | Shared Rust security logic |
| **Min SDK** | API 24 (Android 7.0) | Balance security updates vs. coverage |
| **Target SDK** | API 34 (Android 14) | Latest security features |
| **Networking** | OkHttp + Retrofit | Certificate pinning support |
| **Local DB** | Room | Type-safe SQLite abstraction |
| **Distribution** | Direct APK download | Standard Android sideloading |

#### 2.5.3 iOS Client (Phase 1)

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Language** | Swift | iOS standard |
| **UI Framework** | SwiftUI | Modern declarative UI |
| **Core Integration** | UniFFI to dk-client-core | Shared Rust security logic |
| **Min iOS** | iOS 15 | Balance features vs. coverage (~95% devices) |
| **Networking** | URLSession | Native iOS networking |
| **Local DB** | SwiftData / Core Data | Native iOS persistence |
| **Distribution** | DMA Web Distribution | EU alternative distribution via Apple notarization |

### 2.6 Observability Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Metrics** | Prometheus | Industry standard, pull-based |
| **Visualization** | Grafana | Flexible dashboards, alerting |
| **Logging** | OpenSearch | Scalable, open source Elasticsearch fork |
| **Tracing** | OpenTelemetry + Jaeger | Distributed tracing standard |
| **Uptime Monitoring** | Uptime Kuma | Self-hosted, simple |

### 2.7 Language Decision Summary

| Component | Language | Rationale |
|-----------|----------|-----------|
| API Server | **Rust** | Security-critical, high performance |
| Signing Service | **Rust** | HSM integration, memory safety paramount |
| Build Workers | **Rust** | Safe concurrency, hash verification |
| Security Scanners | **Rust** (orchestration) | Coordination layer; calls external tools |
| Client Core | **Rust** | Shared security logic across iOS/Android (ADR-002) |
| fdroidserver Integration | **Python** | Subprocess interface to existing tooling |
| Android Client UI | **Kotlin** | Android standard, F-Droid upstream compatibility |
| iOS Client UI | **Swift** | iOS standard, SwiftUI for modern UI |
| Infrastructure/Scripts | **Bash/Python** | Tooling, automation, non-critical paths |

---

## 3. Repository Structure

```
dk-appstore/
├── .gitlab-ci.yml                 # CI/CD pipeline definition
├── LICENSE                        # EUPL-1.2 for server, Apache-2.0 for client
├── README.md                      # Project overview
├── CONTRIBUTING.md                # Contribution guidelines
├── SECURITY.md                    # Security policy and reporting
├── CODE_OF_CONDUCT.md             # Community guidelines
│
├── docs/                          # Documentation
│   ├── architecture/              # Architecture decision records (ADRs)
│   ├── api/                       # API documentation
│   ├── deployment/                # Deployment guides
│   ├── security/                  # Security documentation
│   └── user-guides/               # End-user documentation
│
├── server/                        # Repository server (Rust)
│   ├── Cargo.toml                # Workspace root
│   ├── Cargo.lock                # Dependency lock file
│   ├── dk-api/                   # API server crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # Application entry point
│   │       ├── lib.rs            # Library root
│   │       ├── routes/           # axum route handlers
│   │       ├── models/           # Domain models
│   │       ├── services/         # Business logic
│   │       ├── db/               # Database layer (sqlx)
│   │       └── error.rs          # Error types
│   ├── dk-signing/               # Signing service crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── hsm.rs            # HSM/PKCS#11 integration
│   │       └── keys.rs           # Key management
│   ├── dk-build/                 # Build orchestration crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── worker.rs         # Build job worker
│   │       ├── verify.rs         # Reproducibility verification
│   │       └── sbom.rs           # SBOM generation
│   ├── dk-scanner/               # Security scanning orchestration
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── mobsf.rs          # MobSF integration
│   │       └── report.rs         # Report generation
│   ├── dk-common/                # Shared types and utilities
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs         # Configuration
│   │       └── types.rs          # Common types
│   ├── migrations/               # Database migrations (sqlx)
│   ├── tests/                    # Integration tests
│   └── Containerfile             # Container build definition
│
├── fdroid-bridge/                 # Python fdroidserver integration
│   ├── pyproject.toml            # Project config (uv compatible, no requirements.txt)
│   ├── .python-version           # Python version pinning
│   ├── fdroid_bridge/            # Main package
│   │   ├── __init__.py
│   │   ├── index.py              # Index generation wrapper
│   │   ├── metadata.py           # Metadata handling
│   │   └── errors.py             # Error types
│   └── tests/                    # pytest tests
│
├── client-core/                   # Shared client core (Rust) - ADR-002
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                # Library root
│       ├── repository.rs         # Repository management
│       ├── index.rs              # Index parsing and caching
│       ├── verification.rs       # Signature verification
│       ├── crypto.rs             # Cryptographic utilities
│       ├── download.rs           # Download management
│       ├── update.rs             # Update checking logic
│       └── uniffi.udl            # UniFFI interface definition
│
├── client-android/                # Android client application
│   ├── app/                       # Main application module
│   │   ├── src/
│   │   │   ├── main/
│   │   │   │   ├── kotlin/       # Kotlin source
│   │   │   │   ├── rust/         # JNI bindings to client-core
│   │   │   │   ├── res/          # Resources
│   │   │   │   └── AndroidManifest.xml
│   │   │   └── test/             # Unit tests
│   │   └── build.gradle.kts
│   ├── gradle/                    # Gradle wrapper and verification
│   ├── build.gradle.kts          # Root build config
│   ├── settings.gradle.kts
│   └── gradle.properties
│
├── client-ios/                    # iOS client application (Phase 1)
│   ├── DKAppStore/
│   │   ├── App/                  # SwiftUI app structure
│   │   ├── Views/                # SwiftUI views
│   │   ├── ViewModels/           # MVVM view models
│   │   └── RustBridge/           # UniFFI generated Swift bindings
│   ├── DKAppStore.xcodeproj
│   └── Package.swift
│
├── build-system/                  # Reproducible build infrastructure
│   ├── images/                    # Container images for builds
│   │   ├── android-builder/      # Android SDK build environment
│   │   └── verifier/             # Build verification environment
│   ├── scripts/                   # Build automation scripts
│   │   ├── build-apk.sh          # APK build script
│   │   ├── verify-reproducible.sh # Reproducibility check
│   │   └── sign-release.sh       # Signing orchestration
│   └── configs/                   # Build configurations
│
├── security/                      # Security tooling and configs
│   ├── scanning/                  # Static analysis configs
│   │   ├── mobsf-config.yaml
│   │   ├── semgrep-rules/
│   │   └── bandit.yaml
│   ├── signing/                   # Signing infrastructure
│   │   ├── softhsm/              # Development HSM simulation
│   │   └── scripts/              # Key ceremony scripts
│   └── policies/                  # Security policies (OPA)
│
├── infrastructure/                # Infrastructure as Code
│   ├── kubernetes/               # K8s manifests
│   │   ├── base/                 # Base configurations
│   │   └── overlays/             # Environment-specific
│   │       ├── dev/
│   │       ├── staging/
│   │       └── prod/
│   ├── terraform/                # Cloud infrastructure (optional)
│   └── ansible/                  # Configuration management
│
├── monitoring/                    # Observability configuration
│   ├── prometheus/               # Prometheus rules and alerts
│   ├── grafana/                  # Dashboard definitions
│   └── alertmanager/             # Alert routing
│
└── tools/                         # Development utilities
    ├── dev-setup.sh              # Local development setup
    ├── pre-commit-config.yaml    # Pre-commit hooks
    └── scripts/                  # Utility scripts
```

---

## 4. Phase 0 Milestones

### Milestone 1: Project Bootstrap (Weeks 1-3)

**Deliverables:**
- [ ] Repository initialized with structure above
- [ ] Development environment documentation
- [ ] CI/CD pipeline skeleton (lint, test, build)
- [ ] Pre-commit hooks configured (gitleaks, formatting)
- [ ] CONTRIBUTING.md and SECURITY.md published
- [ ] Initial ADR (Architecture Decision Record) for tech stack

**Acceptance Criteria:**
- Contributors can clone and run `./tools/dev-setup.sh` to get a working environment
- CI pipeline runs on every merge request
- Code formatting is enforced automatically

### Milestone 2: Core API Server in Rust (Weeks 4-8)

**Deliverables:**
- [ ] Rust workspace structure with crates (dk-api, dk-common)
- [ ] axum-based API server with health endpoints
- [ ] PostgreSQL integration with sqlx (compile-time verified queries)
- [ ] fdroid-bridge Python module for index generation
- [ ] Container image for API server
- [ ] Basic test suite (>80% coverage on new code)

**Acceptance Criteria:**
- Can add an APK to the repository via API
- Repository index is generated via fdroid-bridge and signed
- F-Droid client can connect and browse apps

**Key API Endpoints (v1):**
```
GET  /api/v1/index                 # Repository index
GET  /api/v1/apps                  # List applications
GET  /api/v1/apps/{package_id}     # Application details
GET  /api/v1/apps/{package_id}/versions  # Version history
POST /api/v1/apps (admin)          # Submit new application
GET  /health                       # Health check
GET  /metrics                      # Prometheus metrics
```

**Rust-Specific Quality Gates:**
- `cargo clippy` passes with no warnings
- `cargo audit` reports no vulnerabilities
- `cargo deny check` passes (licenses, duplicates)
- No `unsafe` code in application crates

### Milestone 3: Build System in Rust (Weeks 9-14)

**Deliverables:**
- [ ] dk-build crate with async job worker
- [ ] Ephemeral build container orchestration (Podman)
- [ ] Gradle dependency verification enabled
- [ ] SBOM generation for all builds (CycloneDX)
- [ ] Single-builder reproducible build proof-of-concept
- [ ] Build log capture and storage
- [ ] Redis-based job queue with tokio

**Acceptance Criteria:**
- Can submit source code URL and receive built APK
- Build logs are accessible via API
- SBOM is generated in CycloneDX format
- Same source produces identical APK hash on re-build
- Build worker handles concurrent jobs safely (Rust guarantees)

### Milestone 4: Security Scanning Pipeline (Weeks 15-18)

**Deliverables:**
- [ ] dk-scanner crate for scan orchestration
- [ ] MobSF integration for APK analysis
- [ ] semgrep rules for Android security patterns
- [ ] Dependency vulnerability scanning
- [ ] Security report generation (JSON + human-readable)
- [ ] Blocking rules for critical vulnerabilities
- [ ] API for scan status and results

**Acceptance Criteria:**
- Every build triggers security scan automatically
- Critical vulnerabilities block publication
- Scan results stored and queryable
- OWASP Top 10 Mobile covered by rules

### Milestone 5: Shared Client Core & Android Client (Weeks 12-20)

**Deliverables:**
- [ ] `client-core` Rust crate with FFI-friendly API
- [ ] Signature verification in Rust (portable across platforms)
- [ ] Index parsing and caching in Rust
- [ ] JNI bindings for Android integration
- [ ] F-Droid client forked and building with Rust core
- [ ] Danish branding and localization (da_DK)
- [ ] Hardcoded DK-AppStore repository endpoint
- [ ] Certificate pinning via rustls in client-core
- [ ] Offline index caching functional
- [ ] Basic UI customization (colors, logo)

**Acceptance Criteria:**
- Client installs on Android 7.0+ devices
- Can browse and install apps from test repository
- Works offline with cached index
- Passes basic security audit (no hardcoded secrets, pinning works)
- Rust client-core compiles for Android ARM64/ARM32/x86_64
- All security-critical operations use Rust core (not Kotlin)

**Client Core API (Rust):**
```rust
// Key exports from client-core
pub fn verify_index_signature(index: &[u8], signature: &[u8]) -> Result<bool, VerifyError>;
pub fn verify_apk_signature(apk_path: &Path) -> Result<ApkSignatureInfo, VerifyError>;
pub fn parse_index(data: &[u8]) -> Result<RepositoryIndex, ParseError>;
pub fn check_certificate_pin(cert: &[u8], expected_pins: &[Pin]) -> Result<bool, PinError>;
```

### Milestone 6: Integration & Documentation (Weeks 21-24)

**Deliverables:**
- [ ] End-to-end flow: submit → build → scan → sign → publish → install
- [ ] dk-signing crate with SoftHSM2 integration (cryptoki)
- [ ] Comprehensive API documentation (OpenAPI via utoipa)
- [ ] Deployment guide for staging environment
- [ ] Security documentation for handover
- [ ] Load testing baseline (target: 1000 concurrent downloads)

**Acceptance Criteria:**
- Full workflow demonstrable in staging environment
- Documentation sufficient for vendor onboarding
- No critical/high vulnerabilities in own codebase
- `cargo audit` clean, `cargo deny` passing
- Performance baseline documented

---

## 5. Development Workflow

### 5.1 Branching Strategy

```
main                    # Protected, requires MR + approval
├── develop             # Integration branch
├── feature/XXX-desc    # Feature branches
├── fix/XXX-desc        # Bug fixes
└── release/v0.1.0      # Release branches
```

### 5.2 Merge Request Requirements

- [ ] All CI checks pass (lint, test, security scan)
- [ ] Code review by at least 1 maintainer
- [ ] Security-sensitive changes require 2 reviewers
- [ ] Documentation updated if API changes
- [ ] No decrease in test coverage

### 5.3 Release Process

1. Create release branch from `develop`
2. Version bump and changelog update
3. Security scan of release candidate
4. Tag and sign release
5. Publish container images
6. Update documentation

---

## 6. Security Considerations (Phase 0)

### 6.1 Development Environment Security

| Control | Implementation |
|---------|----------------|
| Secret scanning | Gitleaks pre-commit + CI |
| Dependency pinning | Exact versions in lock files |
| Signed commits | Required for maintainers |
| Access control | GitLab protected branches |
| Vulnerability disclosure | SECURITY.md with contact |

### 6.2 Signing Key Management (Development)

For Phase 0, we use SoftHSM2 to simulate HSM operations:

```bash
# Initialize SoftHSM slot
softhsm2-util --init-token --slot 0 --label "DK-AppStore-Dev"

# Generate signing key (development only)
pkcs11-tool --module /usr/lib/softhsm/libsofthsm2.so \
  --login --keypairgen --key-type EC:secp256r1 \
  --label "repo-signing-key-dev"
```

**Note:** Production keys will be generated in hardware HSM during Phase 1 key ceremony.

### 6.3 Known Limitations (Phase 0)

| Limitation | Mitigation | Resolution Timeline |
|------------|------------|---------------------|
| Single-builder (not triple) | Document reproducibility approach | Phase 1 |
| SoftHSM not hardware HSM | Clearly marked as non-production | Phase 1 |
| No air-gap | Development convenience | Phase 1 |
| Limited penetration testing | Community review, basic scanning | Phase 1 |

---

## 7. Team Structure (Recommended)

### 7.1 Core Team (DIGST) - Phase 0

| Role | Responsibility | FTE |
|------|----------------|-----|
| Technical Lead | Architecture decisions, vendor coordination, Rust expertise | 1.0 |
| Rust Developer (Senior) | Server, API, signing service, client-core | 1.0 |
| Rust Developer | Build system, security scanner orchestration | 1.0 |
| Android Developer | Client application (Kotlin), JNI integration | 0.5 |
| Security Engineer | Scanning pipeline, code reviews, threat modeling | 0.5 |
| DevOps Engineer | CI/CD, infrastructure, container builds | 0.5 |

**Note:** Team composition reflects Rust as primary language. Budget for Rust training for existing staff or hire experienced Rust developers.

### 7.1.1 Additional Team (Phase 1 - iOS)

| Role | Responsibility | FTE |
|------|----------------|-----|
| iOS Developer | SwiftUI client, UniFFI integration | 0.5-1.0 |
| Rust Developer | UniFFI bindings, iOS cross-compilation | 0.5 (from existing team) |

**Note:** iOS developer should have interest in Rust or willingness to learn FFI patterns. Consider contractor with Swift + systems programming background.

### 7.2 Community Contributors

- Code contributions via merge requests
- Security research and vulnerability reports
- Documentation and translations
- Testing on diverse Android devices
- Rust ecosystem expertise and crate recommendations

---

## 8. Success Criteria for Phase 0

| Metric | Target |
|--------|--------|
| Repository server operational | Yes |
| Android client app functional | Yes |
| End-to-end flow demonstrated | Yes |
| Test coverage (new code) | >70% |
| Critical vulnerabilities | 0 |
| Documentation completeness | Sufficient for vendor handover |
| Community contributions | At least 3 external contributors |
| Client-core compiles for iOS | Yes (preparation for Phase 1) |

---

## 8.1 Phase 1: iOS Client (Post-Android Launch)

> See [ADR-002: iOS Distribution Strategy](docs/architecture/adr-002-ios-distribution-strategy.md) for full rationale.

**Timeline:** 6 months after Android launch  
**Objective:** Native iOS client using shared Rust core, distributed via DMA Web Distribution

### Phase 1 Milestones

#### Milestone 1.1: UniFFI Bindings (Months 1-2)

**Deliverables:**
- [ ] UniFFI interface definition (uniffi.udl) for client-core
- [ ] Swift bindings generation automated in CI
- [ ] iOS-compatible Rust compilation (aarch64-apple-ios, x86_64-apple-ios)
- [ ] XCFramework packaging for Swift consumption

**Acceptance Criteria:**
- Swift can call all client-core functions
- Rust core tests pass on iOS simulator
- Memory safety maintained across FFI boundary

#### Milestone 1.2: iOS UI Shell (Months 2-3)

**Deliverables:**
- [ ] SwiftUI application structure
- [ ] Repository browsing UI
- [ ] App detail views
- [ ] Settings and preferences
- [ ] Danish localization (da_DK)
- [ ] Accessibility compliance

**Acceptance Criteria:**
- UI matches Android client functionality
- Works on iOS 15+ devices
- Passes Apple Human Interface Guidelines review

#### Milestone 1.3: Apple Developer Setup (Month 3)

**Deliverables:**
- [ ] Apple Developer Program enrollment
- [ ] Signing certificates and provisioning profiles
- [ ] Notarization workflow established
- [ ] EU Alternative Distribution terms accepted

**Acceptance Criteria:**
- Can submit IPA for notarization
- Notarization completes successfully
- App installs via web distribution link

#### Milestone 1.4: DMA Web Distribution (Month 4)

**Deliverables:**
- [ ] Web distribution endpoint on DK-AppStore
- [ ] IPA hosting infrastructure
- [ ] Install flow documentation for users
- [ ] "Enable web installs" user guide

**Acceptance Criteria:**
- Users can install iOS client from dk-appstore.dk
- Installation works after enabling web distribution setting
- MitID (test version) installs and runs

#### Milestone 1.5: iOS Beta & Launch (Months 5-6)

**Deliverables:**
- [ ] Beta testing with Danish users
- [ ] Performance optimization
- [ ] Security audit of iOS-specific code
- [ ] Public launch

**Acceptance Criteria:**
- 1000+ beta testers
- No critical bugs
- Security audit passed
- MitID available on iOS via DK-AppStore

### Phase 1 Success Criteria

| Metric | Target |
|--------|--------|
| iOS client functional | Yes |
| Parity with Android features | 100% |
| Shared code percentage | >70% |
| iOS 15+ device coverage | >95% of Danish iOS users |
| App Store independence | Yes (DMA distribution only) |

---

## 9. Risks and Mitigations

### 9.1 Phase 0 Risks (Android)

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Rust learning curve slows development | Medium | Medium | Training budget, pair programming, hire experienced Rust devs |
| Rust talent acquisition challenges | Medium | Medium | Competitive compensation, remote-friendly, sponsor Rust events |
| fdroidserver breaking changes | Medium | High | Pin version, isolate via fdroid-bridge subprocess interface |
| Insufficient community engagement | Medium | Medium | Active outreach, clear contribution paths |
| Scope creep | High | Medium | Strict milestone definitions, defer to Phase 1 |
| Key personnel departure | Low | High | Documentation-first, knowledge sharing, code review culture |
| Upstream F-Droid client divergence | Medium | Medium | Minimize custom changes, upstream fixes |
| Crate dependency vulnerabilities | Low | Medium | cargo-audit in CI, cargo-deny for license/duplicate checks |

### 9.2 Phase 1 Risks (iOS)

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Apple changes DMA terms | Medium | High | Monitor EU regulatory developments; maintain App Store fallback option |
| Apple notarization rejection | Low | High | Notarization is malware check only; government apps low risk |
| UniFFI limitations block features | Low | Medium | Mozilla actively maintains; cbindgen as fallback |
| User friction from web install toggle | Medium | Medium | Clear documentation; media campaign for MitID launch |
| Rust iOS toolchain issues | Low | Medium | Well-documented; cargo-lipo and xcodebuild integration mature |
| Swift/iOS expertise gap | Medium | Medium | Hire iOS developer with Rust interest; training budget |
| DMA scope narrowing | Low | High | Engage with EU regulators; track EC enforcement actions |

---

## 10. Next Steps

1. **Immediate (This Week)**
   - Initialize Git repository with base structure
   - Set up GitLab CI pipeline skeleton
   - Create CONTRIBUTING.md and SECURITY.md

2. **Week 2-3**
   - Development environment setup scripts
   - fdroidserver installation and basic configuration
   - First ADR documenting technology decisions

3. **Ongoing**
   - Weekly progress updates to stakeholders
   - Community engagement and contributor onboarding
   - Security review of all merge requests

---

## Appendix A: Reference Commands

### Local Development Setup

```bash
# Clone repository
git clone https://gitlab.com/digst/dk-appstore.git
cd dk-appstore

# Install Rust (if not present)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup component add clippy rustfmt

# Install uv (Python package manager)
curl -LsSf https://astral.sh/uv/install.sh | sh

# Run setup script (installs dependencies, configures environment)
./tools/dev-setup.sh

# Start local development environment
podman-compose up -d

# Run Rust tests
cd server && cargo test

# Run Rust lints
cargo clippy --all-targets --all-features -- -D warnings

# Run security audit
cargo audit
cargo deny check

# Setup Python environment (fdroid-bridge)
cd fdroid-bridge
uv sync                    # Creates venv and installs deps
uv run ty check            # Type checking
uv run ruff check .        # Linting
uv run pytest              # Run tests

# Run client tests
cd ../client && ./gradlew test

# Run security scan
./security/scanning/run-local-scan.sh
```

### Build Server Locally

```bash
# Debug build
cd server
cargo build

# Release build (optimized)
cargo build --release

# Run API server
cargo run --bin dk-api
```

### Build APK Locally

```bash
# Build client APK
cd client
./gradlew assembleRelease

# Verify reproducibility
./build-system/scripts/verify-reproducible.sh client/app/build/outputs/apk/release/
```

---

## Appendix B: External Dependencies

### Rust Crates (Primary)

| Crate | License | Security Audit Status |
|-------|---------|----------------------|
| axum | MIT | Tokio ecosystem, widely used |
| tokio | MIT | Industry standard async runtime |
| sqlx | MIT/Apache-2.0 | Compile-time query verification |
| serde | MIT/Apache-2.0 | De facto standard, extensively reviewed |
| ring | ISC | BoringSSL-derived, security-focused |
| rustls | MIT/Apache-2.0 | Pure Rust TLS, audited |
| cryptoki | Apache-2.0 | PKCS#11 bindings |
| tracing | MIT | Tokio ecosystem |

### Other Dependencies

| Dependency | License | Security Audit Status |
|------------|---------|----------------------|
| fdroidserver | AGPL-3.0 | Community reviewed |
| F-Droid Client | GPL-3.0 | Community reviewed |
| MobSF | GPL-3.0 | Active security tool |
| PostgreSQL | PostgreSQL License | Extensive security track record |
| Redis | BSD-3-Clause | Widely deployed |

---

## Appendix C: Rust Code Standards

### Linting Configuration

```toml
# .cargo/config.toml
[build]
rustflags = ["-D", "warnings"]

# Cargo.toml (workspace)
[workspace.lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
```

### Minimum Supported Rust Version

- **MSRV:** 1.75.0 (stable)
- **Policy:** Current stable minus 2 versions
- **Enforcement:** `rust-version` in Cargo.toml, CI checks

### Code Review Checklist (Rust-Specific)

- [ ] No `unsafe` blocks (or justified and isolated)
- [ ] No `.unwrap()` or `.expect()` in library code
- [ ] Error types implement `std::error::Error`
- [ ] Public APIs have documentation
- [ ] `cargo clippy` passes without warnings
- [ ] `cargo fmt` applied
- [ ] `cargo audit` shows no vulnerabilities
- [ ] `cargo deny check` passes

---

## Appendix D: Python Code Standards (fdroid-bridge)

### Tooling Requirements

| Tool | Purpose | Command |
|------|---------|---------|
| **uv** | Package management, venv | `uv sync`, `uv add`, `uv run` |
| **ty** | Type checking | `uv run ty check` |
| **ruff** | Linting + formatting | `uv run ruff check .`, `uv run ruff format .` |
| **pytest** | Testing | `uv run pytest` |

### Project Configuration

All Python configuration in `pyproject.toml` only. No `setup.py`, `requirements.txt`, or other config files.

```toml
# pyproject.toml example
[project]
name = "fdroid-bridge"
requires-python = ">=3.11"

[tool.uv]
dev-dependencies = ["pytest", "ruff", "ty"]

[tool.ruff]
target-version = "py311"
select = ["E", "W", "F", "I", "B", "ANN"]  # ANN = require type annotations

[tool.ruff.lint]
select = ["ANN"]  # Enforce type annotations
```

### Virtual Environment

Always use venv managed by uv:

```bash
cd fdroid-bridge
uv sync          # Creates .venv and installs all deps
uv run <cmd>     # Run commands in venv
uv add <pkg>     # Add dependency (updates pyproject.toml)
```

**Never use:**
- `pip install` directly
- `requirements.txt`
- `setup.py`
- Global Python packages

### Type Annotation Requirements

All functions must have complete type annotations:

```python
# Required
def process_app(package_id: str, version: int) -> dict[str, Any]:
    ...

# Not allowed
def process_app(package_id, version):
    ...
```

### Code Review Checklist (Python)

- [ ] All functions have type annotations
- [ ] `uv run ty check` passes
- [ ] `uv run ruff check .` passes
- [ ] `uv run ruff format --check .` passes
- [ ] `uv run pytest` passes
- [ ] Docstrings on public functions
- [ ] No use of pip, requirements.txt, or setup.py

---

*Document maintained by DIGST Technical Team*  
*For questions: [contact to be added]*
