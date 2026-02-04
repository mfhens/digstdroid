//! Error types for DK-AppStore.

use std::fmt;

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Application-level error type.
#[derive(Debug)]
pub enum Error {
    /// Application not found.
    NotFound(String),
    /// Invalid input provided.
    InvalidInput(String),
    /// Database error.
    Database(String),
    /// Configuration error.
    Config(String),
    /// Internal error.
    Internal(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "not found: {msg}"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            Self::Database(msg) => write!(f, "database error: {msg}"),
            Self::Config(msg) => write!(f, "configuration error: {msg}"),
            Self::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Self {
        Self::Config(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::NotFound("app xyz".to_string());
        assert_eq!(err.to_string(), "not found: app xyz");
    }
}
