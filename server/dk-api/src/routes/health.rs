//! Health check endpoints.

use axum::Json;
use serde::Serialize;

/// Health check response.
#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

/// Basic health check endpoint.
///
/// Returns OK if the service is running.
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// Readiness check endpoint.
///
/// Returns OK if the service is ready to accept traffic.
/// In production, this should check database connectivity.
pub async fn readiness_check() -> Json<HealthResponse> {
    // TODO: Check database and Redis connectivity
    Json(HealthResponse {
        status: "ready",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// Liveness check endpoint.
///
/// Returns OK if the service is alive.
/// Used by Kubernetes to determine if the pod should be restarted.
pub async fn liveness_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "alive",
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response.status, "ok");
    }

    #[tokio::test]
    async fn test_readiness_check() {
        let response = readiness_check().await;
        assert_eq!(response.status, "ready");
    }

    #[tokio::test]
    async fn test_liveness_check() {
        let response = liveness_check().await;
        assert_eq!(response.status, "alive");
    }
}
