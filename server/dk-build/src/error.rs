//! Build service error types.

use thiserror::Error;

/// Result type for build operations.
pub type BuildResult<T> = Result<T, BuildError>;

/// Errors that can occur during build operations.
#[derive(Debug, Error)]
pub enum BuildError {
    /// Source repository not found or inaccessible.
    #[error("Source not found: {0}")]
    SourceNotFound(String),

    /// Build configuration is invalid.
    #[error("Invalid build configuration: {0}")]
    InvalidConfig(String),

    /// Build process failed.
    #[error("Build failed: {0}")]
    BuildFailed(String),

    /// Build timed out.
    #[error("Build timed out after {0} seconds")]
    Timeout(u64),

    /// Reproducibility verification failed.
    #[error("Reproducibility check failed: builds do not match")]
    ReproducibilityFailed,

    /// Container orchestration error.
    #[error("Container error: {0}")]
    ContainerError(String),
}
