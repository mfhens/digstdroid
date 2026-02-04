"""Metadata handling for fdroid-bridge."""

import json
from pathlib import Path
from typing import Any

from fdroid_bridge.errors import MetadataNotFoundError, MetadataParseError


def load_metadata(metadata_dir: Path, package_id: str) -> dict[str, Any]:
    """Load application metadata from the metadata directory.

    Args:
        metadata_dir: Path to the metadata directory
        package_id: The package identifier (e.g., "dk.digst.mitid")

    Returns:
        Parsed metadata dictionary

    Raises:
        MetadataNotFoundError: If the metadata file does not exist
        MetadataParseError: If the metadata file is invalid JSON
    """
    metadata_path = metadata_dir / f"{package_id}.json"

    if not metadata_path.exists():
        raise MetadataNotFoundError(f"Metadata not found: {package_id}")

    try:
        content = metadata_path.read_text(encoding="utf-8")
        return json.loads(content)
    except json.JSONDecodeError as e:
        raise MetadataParseError(f"Invalid metadata for {package_id}: {e}") from e


def save_metadata(metadata_dir: Path, package_id: str, metadata: dict[str, Any]) -> None:
    """Save application metadata to the metadata directory.

    Args:
        metadata_dir: Path to the metadata directory
        package_id: The package identifier (e.g., "dk.digst.mitid")
        metadata: Metadata dictionary to save

    Raises:
        OSError: If the file cannot be written
    """
    metadata_dir.mkdir(parents=True, exist_ok=True)
    metadata_path = metadata_dir / f"{package_id}.json"
    content = json.dumps(metadata, indent=2, ensure_ascii=False)
    metadata_path.write_text(content, encoding="utf-8")


def validate_metadata(metadata: dict[str, Any]) -> list[str]:
    """Validate metadata against required fields.

    Args:
        metadata: Metadata dictionary to validate

    Returns:
        List of validation error messages (empty if valid)
    """
    errors: list[str] = []
    required_fields = ["name", "summary", "description", "version_code", "version_name"]

    for field in required_fields:
        if field not in metadata:
            errors.append(f"Missing required field: {field}")
        elif not metadata[field]:
            errors.append(f"Empty required field: {field}")

    if "version_code" in metadata:
        if not isinstance(metadata["version_code"], int):
            errors.append("version_code must be an integer")
        elif metadata["version_code"] < 0:
            errors.append("version_code must be non-negative")

    return errors
