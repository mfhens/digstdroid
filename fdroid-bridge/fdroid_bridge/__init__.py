"""fdroid-bridge: Bridge between DK-AppStore and fdroidserver.

This package provides integration with fdroidserver for:
- Repository index generation
- Metadata format handling
- APK processing utilities
"""

from fdroid_bridge.errors import (
    BridgeError,
    IndexGenerationError,
    MetadataError,
    MetadataNotFoundError,
    MetadataParseError,
)

__version__ = "0.1.0"
__all__ = [
    "BridgeError",
    "IndexGenerationError",
    "MetadataError",
    "MetadataNotFoundError",
    "MetadataParseError",
]
