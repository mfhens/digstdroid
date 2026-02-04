#!/usr/bin/env bash
# DK-AppStore Development Environment Setup
# Usage: ./tools/dev-setup.sh

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Check if command exists
check_cmd() {
    if ! command -v "$1" &> /dev/null; then
        return 1
    fi
    return 0
}

# Header
echo "========================================"
echo "  DK-AppStore Development Setup"
echo "========================================"
echo ""

# Check prerequisites
info "Checking prerequisites..."

# Rust
if check_cmd rustc; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    info "Rust found: $RUST_VERSION"

    # Check minimum version (1.75)
    REQUIRED_VERSION="1.75.0"
    if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
        warn "Rust version $RUST_VERSION is below required $REQUIRED_VERSION"
        warn "Run: rustup update stable"
    fi
else
    error "Rust not found. Install from https://rustup.rs"
fi

# Rust components
info "Installing Rust components..."
rustup component add clippy rustfmt 2>/dev/null || true

# Cargo tools
info "Installing Cargo tools..."
cargo install cargo-audit cargo-deny cargo-watch 2>/dev/null || true

# Podman (optional, warn if missing)
if check_cmd podman; then
    PODMAN_VERSION=$(podman --version | cut -d' ' -f3)
    info "Podman found: $PODMAN_VERSION"
else
    warn "Podman not found. Container builds will not work."
    warn "Install from https://podman.io/getting-started/installation"
fi

# Python (for fdroid-bridge)
if check_cmd python3; then
    PYTHON_VERSION=$(python3 --version | cut -d' ' -f2)
    info "Python found: $PYTHON_VERSION"

    # Check minimum version (3.11)
    REQUIRED_PY_VERSION="3.11"
    PY_MAJOR_MINOR=$(echo "$PYTHON_VERSION" | cut -d'.' -f1,2)
    if [ "$(printf '%s\n' "$REQUIRED_PY_VERSION" "$PY_MAJOR_MINOR" | sort -V | head -n1)" != "$REQUIRED_PY_VERSION" ]; then
        warn "Python version $PYTHON_VERSION is below required $REQUIRED_PY_VERSION"
    fi
else
    warn "Python 3 not found. fdroid-bridge will not work."
fi

# uv (Python package manager)
if check_cmd uv; then
    UV_VERSION=$(uv --version | cut -d' ' -f2)
    info "uv found: $UV_VERSION"
else
    info "Installing uv..."
    if check_cmd curl; then
        curl -LsSf https://astral.sh/uv/install.sh | sh
        # Add to PATH for this session
        export PATH="$HOME/.cargo/bin:$PATH"
        if check_cmd uv; then
            info "uv installed successfully"
        else
            warn "uv installation may require shell restart"
        fi
    else
        warn "curl not found. Install uv manually: https://github.com/astral-sh/uv"
    fi
fi

# Pre-commit (install via uv if available)
if check_cmd pre-commit; then
    info "pre-commit found"
else
    info "Installing pre-commit..."
    if check_cmd uv; then
        uv tool install pre-commit
    elif check_cmd pipx; then
        pipx install pre-commit
    else
        warn "Install pre-commit manually: uv tool install pre-commit"
    fi
fi

# Setup pre-commit hooks
if check_cmd pre-commit; then
    info "Setting up pre-commit hooks..."
    pre-commit install
    pre-commit install --hook-type commit-msg
fi

# Create local configuration
info "Creating local configuration..."
if [ ! -f ".env" ]; then
    cat > .env << 'EOF'
# DK-AppStore Local Development Configuration
# Copy to .env.local and customize
#
# WARNING: These are LOCAL DEVELOPMENT credentials only.
# Never use these values in production environments.

# Database (local dev only - use secrets management in production)
DATABASE_URL=postgres://dk_appstore:dev_password@localhost:5432/dk_appstore_dev

# Redis
REDIS_URL=redis://localhost:6379

# API Server
API_HOST=127.0.0.1
API_PORT=8080
RUST_LOG=dk_api=debug,tower_http=debug

# Signing (development only - uses SoftHSM)
HSM_MODULE=/usr/lib/softhsm/libsofthsm2.so
HSM_SLOT=0
HSM_PIN=dev_pin

# Build system
BUILD_TIMEOUT_SECS=600
EOF
    info "Created .env file (customize as needed)"
else
    info ".env file already exists"
fi

# Create data directories
info "Creating data directories..."
mkdir -p data/{repo,builds,logs}
mkdir -p .cargo

# Cargo configuration for faster builds
if [ ! -f ".cargo/config.toml" ]; then
    cat > .cargo/config.toml << 'EOF'
[build]
# Use all available cores
jobs = -1

[target.x86_64-unknown-linux-gnu]
# Use mold linker if available (faster linking)
# linker = "clang"
# rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[alias]
# Useful aliases
c = "check"
t = "test"
b = "build"
r = "run"
cl = "clippy --all-targets --all-features -- -D warnings"
EOF
    info "Created .cargo/config.toml"
fi

# Build project
info "Building project..."
if [ -f "server/Cargo.toml" ]; then
    cargo build
    info "Build successful!"
else
    warn "server/Cargo.toml not found - skipping build"
    info "Run 'cargo build' after creating Rust workspace"
fi

# Run Rust checks
info "Running Rust checks..."
if [ -f "server/Cargo.toml" ]; then
    cargo fmt --all -- --check || warn "Code formatting issues found. Run 'cargo fmt'"
    cargo clippy --all-targets --all-features -- -D warnings || warn "Clippy warnings found"
fi

# Setup fdroid-bridge Python environment
info "Setting up fdroid-bridge Python environment..."
if [ -d "fdroid-bridge" ] && [ -f "fdroid-bridge/pyproject.toml" ]; then
    if check_cmd uv; then
        cd fdroid-bridge
        uv sync
        info "fdroid-bridge dependencies installed"

        # Run Python checks
        info "Running Python checks..."
        uv run ty check || warn "Type check issues found. Run 'uv run ty check'"
        uv run ruff check . || warn "Ruff issues found. Run 'uv run ruff check .'"
        cd ..
    else
        warn "uv not found. Cannot setup fdroid-bridge environment."
    fi
else
    warn "fdroid-bridge/pyproject.toml not found - skipping Python setup"
fi

# Summary
echo ""
echo "========================================"
echo "  Setup Complete!"
echo "========================================"
echo ""
info "Next steps:"
echo "  1. Start PostgreSQL and Redis (or use podman-compose)"
echo "  2. Run database migrations: cargo run --bin migrate"
echo "  3. Start API server: cargo run --bin dk-api"
echo "  4. Run tests: cargo test"
echo ""
info "Useful commands (Rust):"
echo "  cargo watch -x check    # Auto-check on file changes"
echo "  cargo clippy            # Run linter"
echo "  cargo audit             # Security audit"
echo "  cargo deny check        # License/dependency check"
echo ""
info "Useful commands (Python - fdroid-bridge):"
echo "  cd fdroid-bridge"
echo "  uv sync                 # Install/update dependencies"
echo "  uv run ty check         # Type checking"
echo "  uv run ruff check .     # Linting"
echo "  uv run ruff format .    # Format code"
echo "  uv run pytest           # Run tests"
echo "  uv add <package>        # Add a dependency"
echo ""
