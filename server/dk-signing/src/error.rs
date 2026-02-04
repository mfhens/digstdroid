//! Signing service error types.

use thiserror::Error;

/// Result type for signing operations.
pub type SigningResult<T> = Result<T, SigningError>;

/// Errors that can occur during signing operations.
#[derive(Debug, Error)]
pub enum SigningError {
    /// HSM is not available or not responding.
    #[error("HSM not available: {0}")]
    HsmUnavailable(String),

    /// HSM operation timed out.
    #[error("HSM operation timed out")]
    HsmTimeout,

    /// Authentication to HSM failed.
    #[error("HSM authentication failed")]
    HsmAuthFailed,

    /// Key not found in HSM.
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    /// Invalid key for the requested operation.
    #[error("Invalid key: {0}")]
    InvalidKey(String),

    /// Signing operation failed.
    #[error("Signing failed: {0}")]
    SigningFailed(String),

    /// Verification failed.
    #[error("Signature verification failed")]
    VerificationFailed,
}
