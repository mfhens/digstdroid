"""Tests for metadata handling."""

import json
import tempfile
from pathlib import Path

import pytest

from fdroid_bridge.errors import MetadataNotFoundError, MetadataParseError
from fdroid_bridge.metadata import load_metadata, save_metadata, validate_metadata


class TestLoadMetadata:
    """Tests for load_metadata function."""

    def test_load_valid_metadata(self, tmp_path: Path) -> None:
        """Test loading valid metadata file."""
        metadata = {"name": "Test App", "version_code": 1}
        metadata_path = tmp_path / "dk.test.app.json"
        metadata_path.write_text(json.dumps(metadata), encoding="utf-8")

        result = load_metadata(tmp_path, "dk.test.app")

        assert result == metadata

    def test_load_nonexistent_metadata(self, tmp_path: Path) -> None:
        """Test loading non-existent metadata file raises error."""
        with pytest.raises(MetadataNotFoundError) as exc_info:
            load_metadata(tmp_path, "nonexistent.app")

        assert "nonexistent.app" in str(exc_info.value)

    def test_load_invalid_json(self, tmp_path: Path) -> None:
        """Test loading invalid JSON raises error."""
        metadata_path = tmp_path / "invalid.app.json"
        metadata_path.write_text("not valid json {", encoding="utf-8")

        with pytest.raises(MetadataParseError) as exc_info:
            load_metadata(tmp_path, "invalid.app")

        assert "invalid.app" in str(exc_info.value)


class TestSaveMetadata:
    """Tests for save_metadata function."""

    def test_save_metadata_creates_file(self, tmp_path: Path) -> None:
        """Test saving metadata creates the file."""
        metadata = {"name": "Test App", "version_code": 1}

        save_metadata(tmp_path, "dk.test.app", metadata)

        saved_path = tmp_path / "dk.test.app.json"
        assert saved_path.exists()
        assert json.loads(saved_path.read_text(encoding="utf-8")) == metadata

    def test_save_metadata_creates_directory(self, tmp_path: Path) -> None:
        """Test saving metadata creates parent directory if needed."""
        metadata_dir = tmp_path / "subdir" / "metadata"
        metadata = {"name": "Test App"}

        save_metadata(metadata_dir, "dk.test.app", metadata)

        assert (metadata_dir / "dk.test.app.json").exists()


class TestValidateMetadata:
    """Tests for validate_metadata function."""

    def test_valid_metadata(self) -> None:
        """Test validation passes for valid metadata."""
        metadata = {
            "name": "Test App",
            "summary": "A test application",
            "description": "Full description here",
            "version_code": 1,
            "version_name": "1.0.0",
        }

        errors = validate_metadata(metadata)

        assert errors == []

    def test_missing_required_field(self) -> None:
        """Test validation fails for missing required field."""
        metadata = {"name": "Test App"}

        errors = validate_metadata(metadata)

        assert any("summary" in e for e in errors)

    def test_empty_required_field(self) -> None:
        """Test validation fails for empty required field."""
        metadata = {
            "name": "",
            "summary": "Summary",
            "description": "Description",
            "version_code": 1,
            "version_name": "1.0",
        }

        errors = validate_metadata(metadata)

        assert any("name" in e and "Empty" in e for e in errors)

    def test_invalid_version_code_type(self) -> None:
        """Test validation fails for non-integer version_code."""
        metadata = {
            "name": "Test",
            "summary": "Summary",
            "description": "Description",
            "version_code": "1",  # Should be int
            "version_name": "1.0",
        }

        errors = validate_metadata(metadata)

        assert any("version_code" in e and "integer" in e for e in errors)

    def test_negative_version_code(self) -> None:
        """Test validation fails for negative version_code."""
        metadata = {
            "name": "Test",
            "summary": "Summary",
            "description": "Description",
            "version_code": -1,
            "version_name": "1.0",
        }

        errors = validate_metadata(metadata)

        assert any("non-negative" in e for e in errors)
