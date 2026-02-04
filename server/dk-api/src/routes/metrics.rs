//! Prometheus metrics endpoint.

use axum::response::IntoResponse;

/// Prometheus metrics endpoint.
///
/// Returns metrics in Prometheus text format.
pub async fn metrics_handler() -> impl IntoResponse {
    // TODO: Integrate with metrics-exporter-prometheus
    // For now, return a placeholder
    "# DK-AppStore Metrics\n# TODO: Implement metrics collection\n"
}
