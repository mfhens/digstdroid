# AI-Assisted Development Guidelines

This document establishes guidelines for using AI coding assistants (GitHub Copilot, Claude, GPT-4, etc.) when contributing to DK-AppStore.

## Philosophy

DK-AppStore welcomes AI-assisted development. AI tools can accelerate development, catch bugs, and improve code quality. However, **AI-generated code is held to the same standards as human-written code**. The human contributor remains responsible for all code they submit.

## Core Principles

### 1. Human Accountability

- **You are responsible** for all code you submit, regardless of origin
- **Review everything** AI generates before committing
- **Understand the code** - don't commit code you can't explain
- **Test thoroughly** - AI can generate plausible but incorrect code

### 2. Security First

AI models are trained on public code, which includes vulnerabilities. Be especially vigilant:

```rust
// AI might generate this (WRONG - panics on invalid input)
fn parse_package_id(input: &str) -> &str {
    input.split('.').last().unwrap()
}

// Correct version (handles errors properly)
fn parse_package_id(input: &str) -> Result<&str, ParseError> {
    input.split('.').last().ok_or(ParseError::InvalidFormat)
}
```

### 3. No Secrets in Prompts

**Never** include in AI prompts:
- API keys, tokens, or credentials
- Internal URLs or infrastructure details
- Private keys or certificates
- Personal data or PII
- Security vulnerability details

### 4. Verify Dependencies

AI often suggests popular but potentially problematic dependencies:

```bash
# Always verify AI-suggested dependencies
cargo audit
cargo deny check

# Check crate reputation
# - Downloads on crates.io
# - Recent maintenance activity
# - Known security issues
```

## Quality Requirements

AI-generated code must meet all project standards:

### Rust Code

- [ ] No `unsafe` blocks without justification
- [ ] No `.unwrap()` or `.expect()` in library code
- [ ] Proper error handling with `Result<T, E>`
- [ ] Documentation on public APIs
- [ ] Unit tests for new functionality
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt` applied

### Common AI Mistakes to Catch (Rust)

| Issue | Example | Fix |
|-------|---------|-----|
| **Panicking code** | `.unwrap()`, `.expect()`, `panic!()` | Use `?` operator, return `Result` |
| **Outdated patterns** | `try!()` macro | Use `?` operator |
| **Wrong async runtime** | `async-std` in tokio project | Use `tokio` consistently |
| **Insecure defaults** | Disabled TLS verification | Enable TLS, use `rustls` |
| **Missing validation** | Direct use of user input | Validate and sanitize |
| **Verbose code** | Manual Option handling | Use combinators, `?` |
| **Test gaps** | Missing error case tests | Test happy path AND errors |

### Python Code (fdroid-bridge)

Python is used only for the fdroid-bridge component. AI-generated Python must follow project standards:

- [ ] All functions have complete type annotations
- [ ] Use `uv` for package management (never raw `pip`)
- [ ] Use `ty` for type checking (not mypy)
- [ ] Use `ruff` for linting and formatting
- [ ] Virtual environment managed by `uv sync`
- [ ] Only `pyproject.toml` for configuration

### Common AI Mistakes to Catch (Python)

| Issue | Example | Fix |
|-------|---------|-----|
| **Missing type hints** | `def foo(x):` | `def foo(x: str) -> int:` |
| **Using pip** | `pip install package` | `uv add package` |
| **requirements.txt** | Creating requirements.txt | Use only `pyproject.toml` |
| **No error handling** | Bare `except:` | Specific exception types |
| **Mutable defaults** | `def foo(items=[]):` | `def foo(items: list | None = None):` |
| **Using mypy** | `mypy .` | `uv run ty check` |

### Example: AI-Generated vs. Corrected (Python)

**AI might generate:**
```python
def get_app_metadata(package_id):
    with open(f"metadata/{package_id}.json") as f:
        return json.load(f)
```

**Correct version:**
```python
from pathlib import Path
import json
from typing import Any

from fdroid_bridge.errors import MetadataNotFoundError, MetadataParseError


def get_app_metadata(package_id: str) -> dict[str, Any]:
    """Load application metadata from the metadata directory.

    Args:
        package_id: The package identifier (e.g., "dk.digst.mitid")

    Returns:
        Parsed metadata dictionary

    Raises:
        MetadataNotFoundError: If the metadata file does not exist
        MetadataParseError: If the metadata file is invalid JSON
    """
    metadata_path = Path("metadata") / f"{package_id}.json"

    if not metadata_path.exists():
        raise MetadataNotFoundError(f"Metadata not found: {package_id}")

    try:
        return json.loads(metadata_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as e:
        raise MetadataParseError(f"Invalid metadata for {package_id}: {e}") from e
```

### Example: AI-Generated vs. Corrected

**AI might generate:**
```rust
async fn fetch_app(id: String) -> App {
    let response = reqwest::get(format!("https://api.example.com/apps/{}", id))
        .await
        .unwrap();
    response.json().await.unwrap()
}
```

**Correct version:**
```rust
/// Fetches an application by ID from the repository.
///
/// # Errors
///
/// Returns `ApiError::Network` if the request fails,
/// or `ApiError::NotFound` if the app doesn't exist.
async fn fetch_app(id: &str) -> Result<App, ApiError> {
    let url = format!("https://api.example.com/apps/{}", id);
    
    let response = reqwest::get(&url)
        .await
        .map_err(ApiError::Network)?;
    
    if response.status() == StatusCode::NOT_FOUND {
        return Err(ApiError::NotFound(id.to_string()));
    }
    
    response
        .error_for_status()
        .map_err(ApiError::Network)?
        .json()
        .await
        .map_err(ApiError::Parse)
}
```

## Workflow Integration

### Using AI Effectively

**Good uses:**
- Boilerplate generation (struct definitions, API handlers)
- Test case generation (especially edge cases)
- Documentation drafting
- Code review assistance
- Refactoring suggestions
- Learning unfamiliar APIs

**Risky uses (extra review needed):**
- Cryptographic code
- Authentication/authorization logic
- Input validation
- Error handling
- Concurrent code

**Prohibited uses:**
- Security-critical signing code without expert review
- Direct copy-paste without understanding
- Generating code that handles secrets

### Prompt Engineering Tips

**Be specific about requirements:**
```
Write a Rust function using axum that:
- Handles GET /api/v1/apps/{id}
- Returns Result<Json<App>, ApiError>
- Uses sqlx for database access
- Includes proper error handling (no unwrap)
- Follows the project's error type pattern in dk-common
```

**Request tests:**
```
Also write unit tests covering:
- Successful retrieval
- Not found case
- Database error case
```

**Ask for security review:**
```
Review this code for security issues:
- Input validation
- SQL injection
- Error information leakage
```

## Disclosure

### When to Disclose AI Usage

**Not required:**
- Routine code completion (autocomplete-style)
- Documentation improvements
- Formatting suggestions

**Encouraged:**
- Significant AI-generated implementations
- Complex algorithms or logic
- When AI helped find a solution approach

**Format:**
```
feat(api): add version history endpoint

Implements GET /api/v1/apps/{id}/versions with pagination.

Co-developed with AI assistance (Claude) for initial structure.
Human-reviewed and tested for security and correctness.

Closes #42
```

### Review Transparency

When reviewing AI-assisted PRs:
- Ask clarifying questions about implementation choices
- Verify the contributor understands the code
- Pay extra attention to common AI mistakes

## Prohibited Patterns

### Do Not Use AI For:

1. **Generating signing/HSM code** - Requires expert human implementation
2. **Circumventing security controls** - e.g., disabling TLS for "testing"
3. **Copying code without license verification** - AI may reproduce GPL code
4. **Credentials or secrets** - Even fake/example ones in prompts

### Code That Requires Human Expertise

| Area | Reason |
|------|--------|
| `dk-signing/` | Cryptographic operations require expert review |
| HSM integration | Hardware security is not AI's strength |
| Key derivation | Must follow established standards exactly |
| Authentication flows | Security-critical, subtle bugs have severe impact |

## Tools and Configuration

### Recommended AI Tools

| Tool | Use Case | Notes |
|------|----------|-------|
| GitHub Copilot | In-editor completion | Enable for Rust |
| Claude/GPT-4 | Complex problem solving | Use for design discussions |
| Cursor | AI-native editor | Good for refactoring |

### Editor Configuration

**.vscode/settings.json** (example):
```json
{
  "github.copilot.enable": {
    "*": true,
    "plaintext": false,
    "markdown": true,
    "rust": true
  },
  "github.copilot.advanced": {
    "secretFiltering": true
  }
}
```

### Pre-commit AI Checks

The pre-commit hooks will catch common AI mistakes:
- Secret detection (gitleaks)
- Unsafe code detection
- Unwrap/expect detection in library code

## Resources

### Learning Rust (for AI Users)

If AI is helping you learn Rust:
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings exercises](https://github.com/rust-lang/rustlings)

### AI + Security Resources

- [OWASP AI Security Guidelines](https://owasp.org/www-project-ai-security-and-privacy-guide/)
- [GitHub Copilot Security Best Practices](https://docs.github.com/en/copilot/using-github-copilot/best-practices-for-using-github-copilot)

## Summary Checklist

Before committing AI-assisted code:

### Rust
- [ ] I understand every line of this code
- [ ] I can explain why this approach was chosen
- [ ] No secrets or sensitive data in prompts
- [ ] No `.unwrap()` or `.expect()` in library code
- [ ] No `unsafe` blocks (or justified)
- [ ] Error handling is comprehensive
- [ ] Input validation is present
- [ ] Tests cover happy path and error cases
- [ ] `cargo clippy` passes
- [ ] `cargo audit` passes
- [ ] Documentation is accurate

### Python (fdroid-bridge)
- [ ] All functions have type annotations
- [ ] `uv run ty check` passes
- [ ] `uv run ruff check .` passes
- [ ] `uv run ruff format --check .` passes
- [ ] `uv run pytest` passes
- [ ] No use of pip, requirements.txt, or setup.py
- [ ] Docstrings present on public functions

---

*AI is a tool. You are the engineer. Own your code.*
