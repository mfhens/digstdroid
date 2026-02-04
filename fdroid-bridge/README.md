# fdroid-bridge

Bridge component between DK-AppStore Rust services and fdroidserver.

## Overview

This Python package provides a thin integration layer that:

- Generates F-Droid compatible repository indices
- Handles metadata format conversions
- Interfaces with fdroidserver CLI tools

## Requirements

- Python 3.11+
- [uv](https://github.com/astral-sh/uv) for package management

## Setup

```bash
# Install dependencies (creates venv automatically)
uv sync

# Run type checker
uv run ty check

# Run linter
uv run ruff check .

# Run tests
uv run pytest
```

## Development

### Adding Dependencies

```bash
# Add a runtime dependency
uv add package-name

# Add a dev dependency
uv add --dev package-name
```

### Code Quality

All code must pass:

```bash
uv run ty check           # Type checking
uv run ruff check .       # Linting
uv run ruff format .      # Formatting
uv run pytest             # Tests
```

### Type Annotations

All functions must have complete type annotations:

```python
def process_metadata(package_id: str, version: int) -> dict[str, Any]:
    """Process package metadata."""
    ...
```

## Architecture

```
fdroid-bridge/
├── pyproject.toml        # Project configuration (uv/pip compatible)
├── fdroid_bridge/        # Main package
│   ├── __init__.py
│   ├── index.py          # Index generation
│   ├── metadata.py       # Metadata handling
│   └── errors.py         # Error types
└── tests/                # Test suite
    └── test_*.py
```

## License

EUPL-1.2
