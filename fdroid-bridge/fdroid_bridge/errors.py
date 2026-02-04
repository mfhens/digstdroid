"""Error types for fdroid-bridge."""


class BridgeError(Exception):
    """Base exception for fdroid-bridge errors."""


class MetadataError(BridgeError):
    """Base exception for metadata-related errors."""


class MetadataNotFoundError(MetadataError):
    """Raised when metadata file is not found."""


class MetadataParseError(MetadataError):
    """Raised when metadata file cannot be parsed."""


class IndexGenerationError(BridgeError):
    """Raised when index generation fails."""
