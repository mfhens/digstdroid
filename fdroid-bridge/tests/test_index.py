"""Tests for index generation."""

from pathlib import Path

import pytest

from fdroid_bridge.index import create_minimal_index


class TestCreateMinimalIndex:
    """Tests for create_minimal_index function."""

    def test_creates_valid_structure(self) -> None:
        """Test that minimal index has required structure."""
        apps = [
            {
                "package_id": "dk.test.app",
                "name": "Test App",
                "summary": "A test",
                "description": "Full description",
                "version_code": 1,
            }
        ]

        index = create_minimal_index(
            repo_name="Test Repo",
            repo_description="Test repository",
            apps=apps,
        )

        assert "repo" in index
        assert "apps" in index
        assert "packages" in index
        assert index["repo"]["name"]["en-US"] == "Test Repo"

    def test_includes_app_entries(self) -> None:
        """Test that apps are included in the index."""
        apps = [
            {
                "package_id": "dk.test.app1",
                "name": "App One",
                "summary": "First app",
                "description": "Description one",
                "version_code": 1,
            },
            {
                "package_id": "dk.test.app2",
                "name": "App Two",
                "summary": "Second app",
                "description": "Description two",
                "version_code": 2,
            },
        ]

        index = create_minimal_index(
            repo_name="Test Repo",
            repo_description="Test",
            apps=apps,
        )

        assert "dk.test.app1" in index["apps"]
        assert "dk.test.app2" in index["apps"]
        assert index["apps"]["dk.test.app1"]["name"]["en-US"] == "App One"

    def test_empty_apps_list(self) -> None:
        """Test creating index with no apps."""
        index = create_minimal_index(
            repo_name="Empty Repo",
            repo_description="No apps yet",
            apps=[],
        )

        assert index["apps"] == {}
        assert index["packages"] == {}

    def test_timestamp_is_set(self) -> None:
        """Test that timestamp is included in repo info."""
        index = create_minimal_index(
            repo_name="Test",
            repo_description="Test",
            apps=[],
        )

        assert "timestamp" in index["repo"]
        assert isinstance(index["repo"]["timestamp"], int)
        assert index["repo"]["timestamp"] > 0
