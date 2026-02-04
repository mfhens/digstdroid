//! DK-AppStore API Server
//!
//! The main entry point for the DK-AppStore repository API.

use std::net::SocketAddr;

use axum::{routing::get, Router};
use clap::Parser;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod routes;

use routes::{health, metrics};

/// DK-AppStore API Server
#[derive(Parser, Debug)]
#[command(name = "dk-api")]
#[command(about = "DK-AppStore Repository API Server")]
struct Args {
    /// Host to bind to
    #[arg(long, env = "API_HOST", default_value = "127.0.0.1")]
    host: String,

    /// Port to listen on
    #[arg(long, env = "API_PORT", default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "dk_api=debug,tower_http=debug,axum::rejection=trace".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse command line arguments
    let args = Args::parse();

    // Build application
    let app = create_app();

    // Start server
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;
    info!("Starting DK-AppStore API server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Create the application router.
fn create_app() -> Router {
    Router::new()
        // Health and metrics endpoints
        .route("/health", get(health::health_check))
        .route("/health/ready", get(health::readiness_check))
        .route("/health/live", get(health::liveness_check))
        .route("/metrics", get(metrics::metrics_handler))
        // API v1 routes
        .nest("/api/v1", api_v1_routes())
        // Middleware
        .layer(TraceLayer::new_for_http())
}

/// API v1 routes.
fn api_v1_routes() -> Router {
    Router::new()
        .route("/apps", get(routes::apps::list_apps))
        .route("/apps/:package_id", get(routes::apps::get_app))
        .route(
            "/apps/:package_id/versions",
            get(routes::apps::get_app_versions),
        )
        .route("/index", get(routes::index::get_index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_app();

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).expect("request"))
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_not_found() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/nonexistent")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
