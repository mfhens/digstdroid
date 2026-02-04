//! Error handling for the API.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// API error type.
#[derive(Debug)]
pub enum ApiError {
    /// Resource not found.
    NotFound(String),
    /// Invalid request.
    BadRequest(String),
    /// Internal server error.
    Internal(String),
}

/// Error response body.
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
            Self::Internal(msg) => {
                // Log internal errors but don't expose details
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "An internal error occurred".to_string(),
                )
            }
        };

        let body = ErrorResponse {
            error: error_type.to_string(),
            message,
        };

        (status, Json(body)).into_response()
    }
}

impl From<dk_common::Error> for ApiError {
    fn from(err: dk_common::Error) -> Self {
        match err {
            dk_common::Error::NotFound(msg) => Self::NotFound(msg),
            dk_common::Error::InvalidInput(msg) => Self::BadRequest(msg),
            dk_common::Error::Database(msg) => Self::Internal(msg),
            dk_common::Error::Config(msg) => Self::Internal(msg),
            dk_common::Error::Internal(msg) => Self::Internal(msg),
        }
    }
}
