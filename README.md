# DK-AppStore

**A Sovereign Danish App Distribution Platform**

[![License: EUPL-1.2](https://img.shields.io/badge/License-EUPL--1.2-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

DK-AppStore is an open-source app distribution platform for critical Danish government applications, built on F-Droid foundations with enhanced security measures. Operated by Digitaliseringsstyrelsen (Danish Agency for Digital Government).

## Key Features

- **Sovereign Infrastructure** - Danish-hosted, Danish-controlled
- **Reproducible Builds** - Every app verified by multiple independent builders
- **Privacy by Design** - No user tracking, no accounts required
- **Hardware Security** - HSM-based signing with FIPS 140-3 Level 3 compliance
- **Open Source** - Fully auditable codebase

## Project Status

**Phase 0: Open Source Foundation** (In Progress)

See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for detailed roadmap.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for comprehensive system design.

## Repository Structure

```
dk-appstore/
├── server/                 # Rust server components
│   ├── dk-api/            # API server (axum)
│   ├── dk-signing/        # Signing service
│   ├── dk-build/          # Build orchestration
│   ├── dk-scanner/        # Security scanning
│   └── dk-common/         # Shared types
├── fdroid-bridge/         # Python fdroidserver integration
├── client/                # Android client (Kotlin)
├── build-system/          # Reproducible build infrastructure
├── security/              # Security tooling
├── infrastructure/        # Kubernetes/IaC
├── monitoring/            # Observability config
└── docs/                  # Documentation
```

## Quick Start

### Prerequisites

- Rust 1.75+ (`rustup default stable`)
- Podman 4+
- PostgreSQL 16+
- Python 3.11+ (for fdroid-bridge)

### Development Setup

```bash
# Clone repository
git clone https://gitlab.com/digst/dk-appstore.git
cd dk-appstore

# Run setup script
./tools/dev-setup.sh

# Start local services
podman-compose up -d

# Run tests
cargo test

# Run lints
cargo clippy --all-targets -- -D warnings
```

## Contributing

We welcome contributions! Please read:

- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [SECURITY.md](SECURITY.md) - Security policy
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) - Community guidelines
- [AGENTS.md](AGENTS.md) - AI-assisted development guidelines

## Security

For security vulnerabilities, please follow our [Security Policy](SECURITY.md).

**Do not** open public issues for security vulnerabilities.

## License

- Server components: [EUPL-1.2](LICENSE)
- Android client: Apache-2.0 (see client/LICENSE)
- Documentation: CC-BY-4.0

## Contact

- **Digitaliseringsstyrelsen**: [digst.dk](https://digst.dk)
- **Security Issues**: security@digst.dk (PGP key in SECURITY.md)

---

*Building trustworthy public digital infrastructure for Denmark.*
