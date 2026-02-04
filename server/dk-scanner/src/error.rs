//! Scanner service error types.

use thiserror::Error;

/// Result type for scan operations.
pub type ScanResult<T> = Result<T, ScanError>;

/// Errors that can occur during scanning operations.
#[derive(Debug, Error)]
pub enum ScanError {
    /// APK file not found or inaccessible.
    #[error("APK not found: {0}")]
    ApkNotFound(String),

    /// APK is invalid or corrupted.
    #[error("Invalid APK: {0}")]
    InvalidApk(String),

    /// Scan tool failed.
    #[error("Scan tool failed: {0}")]
    ToolFailed(String),

    /// Scan timed out.
    #[error("Scan timed out after {0} seconds")]
    Timeout(u64),

    /// Critical vulnerability found.
    #[error("Critical vulnerability found: {0}")]
    CriticalVulnerability(String),
}
