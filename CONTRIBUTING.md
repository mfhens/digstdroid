# Contributing to DK-AppStore

Thank you for your interest in contributing to DK-AppStore! This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Security Considerations](#security-considerations)
- [AI-Assisted Development](#ai-assisted-development)

## Code of Conduct

This project follows our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.75+ with clippy and rustfmt
- Podman 4+
- PostgreSQL 16+
- Python 3.11+ (for fdroid-bridge)
- [uv](https://github.com/astral-sh/uv) (Python package manager)
- Git with GPG signing configured

### Setup

```bash
# Fork and clone the repository
git clone https://gitlab.com/YOUR_USERNAME/dk-appstore.git
cd dk-appstore

# Add upstream remote
git remote add upstream https://gitlab.com/digst/dk-appstore.git

# Run development setup
./tools/dev-setup.sh

# Verify Rust setup
cargo build
cargo test
cargo clippy

# Verify Python setup (fdroid-bridge)
cd fdroid-bridge
uv sync
uv run ty check
```

### Understanding the Codebase

1. Read [ARCHITECTURE.md](ARCHITECTURE.md) for system design
2. Read [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for current phase goals
3. Review [ADR documents](docs/architecture/) for key decisions

## Development Workflow

### Branching Strategy

```
main                    # Protected, production-ready
├── develop             # Integration branch
├── feature/XXX-desc    # Feature branches
├── fix/XXX-desc        # Bug fixes
└── release/vX.Y.Z      # Release branches
```

### Creating a Branch

```bash
# Sync with upstream
git fetch upstream
git checkout develop
git merge upstream/develop

# Create feature branch
git checkout -b feature/123-add-endpoint
```

### Making Changes

1. Write tests first (TDD encouraged)
2. Implement the feature
3. Ensure all checks pass:

```bash
# Format code
cargo fmt

# Run lints
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test

# Security audit
cargo audit
cargo deny check

# Pre-commit hooks (if installed)
pre-commit run --all-files
```

## Code Standards

### Rust Code

**Required:**
- No `unsafe` code without explicit justification and review
- No `.unwrap()` or `.expect()` in library code (use proper error handling)
- All public APIs must have documentation
- `cargo fmt` applied
- `cargo clippy` passes with no warnings

**Style:**
```rust
// Good: Explicit error handling
fn process_app(id: &str) -> Result<App, AppError> {
    let app = repository.find(id).ok_or(AppError::NotFound)?;
    Ok(app)
}

// Bad: Panics in library code
fn process_app(id: &str) -> App {
    repository.find(id).unwrap()  // Don't do this
}
```

**Error Types:**
```rust
// Define explicit error types
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Application not found: {0}")]
    NotFound(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
```

### Documentation

- All public functions, structs, and modules need doc comments
- Use `///` for item documentation
- Include examples in doc comments where helpful

```rust
/// Retrieves an application by its package ID.
///
/// # Arguments
///
/// * `package_id` - The unique package identifier (e.g., "dk.digst.mitid")
///
/// # Returns
///
/// The application if found, or `AppError::NotFound` if not.
///
/// # Example
///
/// ```
/// let app = repository.get_app("dk.digst.mitid").await?;
/// ```
pub async fn get_app(&self, package_id: &str) -> Result<App, AppError> {
    // ...
}
```

### Tests

- Unit tests in the same file as the code (`#[cfg(test)]` module)
- Integration tests in `tests/` directory
- Use `proptest` for property-based testing where appropriate
- Aim for >80% coverage on new code

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_app_returns_not_found_for_unknown_id() {
        let repo = TestRepository::new();
        let result = repo.get_app("nonexistent").await;
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }
}
```

### Python Code (fdroid-bridge)

Python is used only for the fdroid-bridge component that interfaces with fdroidserver.

**Required:**
- Always use a virtual environment (managed by uv)
- Only `pyproject.toml` for configuration (no `setup.py`, `requirements.txt`, etc.)
- Use [uv](https://github.com/astral-sh/uv) for package management
- Use [ty](https://github.com/astral-sh/ty) for type checking
- Full type annotations on all functions

**Setup:**
```bash
cd fdroid-bridge

# Create venv and install dependencies
uv sync

# Run type checker
uv run ty check

# Run tests
uv run pytest

# Run linter
uv run ruff check .

# Format code
uv run ruff format .
```

**Style:**
```python
# Good: Full type annotations
def parse_package_id(package_string: str) -> tuple[str, str]:
    """Parse a package string into name and version.
    
    Args:
        package_string: Package identifier (e.g., "dk.digst.mitid:1.0.0")
        
    Returns:
        Tuple of (package_name, version)
        
    Raises:
        ValueError: If the package string is invalid
    """
    if ":" not in package_string:
        raise ValueError(f"Invalid package string: {package_string}")
    name, version = package_string.rsplit(":", 1)
    return name, version


# Bad: No type hints, no docstring
def parse_package_id(package_string):
    parts = package_string.split(":")
    return parts[0], parts[1]  # Will crash on invalid input
```

**Python Code Checklist:**
- [ ] All functions have type annotations
- [ ] `uv run ty check` passes with no errors
- [ ] `uv run ruff check .` passes
- [ ] `uv run ruff format --check .` passes
- [ ] Tests exist and pass (`uv run pytest`)
- [ ] No use of `pip` directly (use `uv` commands)

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, no code change
- `refactor`: Code change that neither fixes nor adds
- `test`: Adding or updating tests
- `chore`: Build, CI, dependencies

**Examples:**
```
feat(api): add endpoint for app version history

Implements GET /api/v1/apps/{id}/versions endpoint
returning paginated version history.

Closes #42
```

```
fix(signing): handle HSM timeout gracefully

Previously, HSM timeouts would panic. Now returns
SigningError::HsmTimeout with retry guidance.

Fixes #87
```

### Signed Commits

All commits must be GPG signed:

```bash
git config commit.gpgsign true
git config user.signingkey YOUR_KEY_ID
```

## Pull Request Process

### Before Submitting

**Rust:**
- [ ] Code compiles without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Lints pass (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Security audit passes (`cargo audit`)
- [ ] License check passes (`cargo deny check`)

**Python (if fdroid-bridge changed):**
- [ ] Type check passes (`cd fdroid-bridge && uv run ty check`)
- [ ] Lints pass (`uv run ruff check .`)
- [ ] Code is formatted (`uv run ruff format --check .`)
- [ ] Tests pass (`uv run pytest`)

**General:**
- [ ] Documentation updated if needed
- [ ] Commit messages follow convention
- [ ] Commits are signed

### PR Description Template

```markdown
## Summary
Brief description of changes.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe testing performed.

## Security Considerations
Note any security implications.

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] CHANGELOG updated (if applicable)
```

### Review Process

1. **Automated checks** must pass (CI pipeline)
2. **Code review** by at least 1 maintainer
3. **Security-sensitive changes** require 2 reviewers
4. **Breaking changes** require technical lead approval

### After Merge

- Delete your feature branch
- Sync your fork with upstream

## Security Considerations

### Sensitive Code Paths

The following areas require extra scrutiny:

- `dk-signing/` - HSM and key management
- `dk-build/src/verify.rs` - Reproducibility verification
- Any code handling cryptographic operations
- Authentication and authorization logic

### What Not to Commit

- Private keys, certificates, or secrets
- Hardcoded credentials
- Internal URLs or IP addresses
- Personal data or PII
- Unaudited dependencies

### Reporting Vulnerabilities

See [SECURITY.md](SECURITY.md) for vulnerability reporting process.

**Do not** open public issues for security vulnerabilities.

## AI-Assisted Development

We support AI-assisted development. See [AGENTS.md](AGENTS.md) for:

- Guidelines for using AI coding assistants
- Quality requirements for AI-generated code
- Review process for AI contributions

AI-generated code is held to the same standards as human-written code.

## Getting Help

- **Questions**: Open a discussion in GitLab
- **Bugs**: Open an issue with reproduction steps
- **Security**: Email security@digst.dk

## Recognition

Contributors are recognized in:
- CONTRIBUTORS.md file
- Release notes for significant contributions
- Annual transparency report

---

Thank you for contributing to Denmark's sovereign digital infrastructure!
