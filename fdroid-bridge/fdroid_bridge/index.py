"""Repository index generation for fdroid-bridge."""

import json
import subprocess
import tempfile
from pathlib import Path
from typing import Any

from fdroid_bridge.errors import IndexGenerationError


def generate_index(repo_dir: Path, output_path: Path | None = None) -> dict[str, Any]:
    """Generate F-Droid compatible repository index.

    This function wraps fdroidserver's index generation functionality
    to create a signed repository index.

    Args:
        repo_dir: Path to the repository directory
        output_path: Optional path to write the index JSON

    Returns:
        Generated index as a dictionary

    Raises:
        IndexGenerationError: If index generation fails
    """
    if not repo_dir.exists():
        raise IndexGenerationError(f"Repository directory not found: {repo_dir}")

    try:
        # Use fdroidserver to generate index
        result = subprocess.run(
            ["fdroid", "update", "--pretty", "--nosign"],
            cwd=repo_dir,
            capture_output=True,
            text=True,
            check=False,
        )

        if result.returncode != 0:
            raise IndexGenerationError(f"fdroid update failed: {result.stderr}")

        # Read generated index
        index_path = repo_dir / "repo" / "index-v2.json"
        if not index_path.exists():
            raise IndexGenerationError("Index file not generated")

        index_content = index_path.read_text(encoding="utf-8")
        index_data: dict[str, Any] = json.loads(index_content)

        if output_path:
            output_path.write_text(
                json.dumps(index_data, indent=2, ensure_ascii=False),
                encoding="utf-8",
            )

        return index_data

    except subprocess.SubprocessError as e:
        raise IndexGenerationError(f"Failed to run fdroidserver: {e}") from e
    except json.JSONDecodeError as e:
        raise IndexGenerationError(f"Failed to parse generated index: {e}") from e


def create_minimal_index(
    repo_name: str,
    repo_description: str,
    apps: list[dict[str, Any]],
) -> dict[str, Any]:
    """Create a minimal F-Droid compatible index structure.

    This creates an index without running fdroidserver, useful for
    testing or when fdroidserver is not available.

    Args:
        repo_name: Name of the repository
        repo_description: Description of the repository
        apps: List of application metadata dictionaries

    Returns:
        Index dictionary in F-Droid v2 format
    """
    import time

    timestamp = int(time.time() * 1000)  # F-Droid uses milliseconds

    return {
        "repo": {
            "name": {"en-US": repo_name},
            "description": {"en-US": repo_description},
            "timestamp": timestamp,
            "version": 21,  # Index format version
        },
        "apps": {app["package_id"]: _format_app_entry(app) for app in apps},
        "packages": {app["package_id"]: [] for app in apps},
    }


def _format_app_entry(app: dict[str, Any]) -> dict[str, Any]:
    """Format an application entry for the index.

    Args:
        app: Application metadata dictionary

    Returns:
        Formatted entry for the index
    """
    return {
        "name": {"en-US": app.get("name", "")},
        "summary": {"en-US": app.get("summary", "")},
        "description": {"en-US": app.get("description", "")},
        "license": app.get("license", "Unknown"),
        "categories": app.get("categories", []),
        "suggestedVersionCode": app.get("version_code", 0),
    }
