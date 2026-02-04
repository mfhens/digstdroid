//! Application-related API endpoints.

use axum::{
    extract::Path,
    Json,
};
use serde::Serialize;

use crate::error::ApiError;

/// Response for listing applications.
#[derive(Serialize)]
pub struct AppsListResponse {
    apps: Vec<AppSummary>,
    total: usize,
}

/// Summary of an application.
#[derive(Serialize)]
pub struct AppSummary {
    package_id: String,
    name: String,
    summary: String,
    version_name: String,
    version_code: i64,
}

/// Detailed application information.
#[derive(Serialize)]
pub struct AppDetail {
    package_id: String,
    name: String,
    summary: String,
    description: String,
    version_name: String,
    version_code: i64,
    created_at: String,
    updated_at: String,
}

/// Application version information.
#[derive(Serialize)]
pub struct AppVersionResponse {
    version_name: String,
    version_code: i64,
    sha256: String,
    size: i64,
    min_sdk: i32,
    target_sdk: i32,
    created_at: String,
}

/// List all applications.
///
/// GET /api/v1/apps
pub async fn list_apps() -> Json<AppsListResponse> {
    // TODO: Implement database query
    // For now, return placeholder data
    Json(AppsListResponse {
        apps: vec![],
        total: 0,
    })
}

/// Get a specific application by package ID.
///
/// GET /api/v1/apps/:package_id
pub async fn get_app(Path(package_id): Path<String>) -> Result<Json<AppDetail>, ApiError> {
    // TODO: Implement database query
    // For now, return not found
    Err(ApiError::NotFound(format!(
        "Application not found: {package_id}"
    )))
}

/// Get version history for an application.
///
/// GET /api/v1/apps/:package_id/versions
pub async fn get_app_versions(
    Path(package_id): Path<String>,
) -> Result<Json<Vec<AppVersionResponse>>, ApiError> {
    // TODO: Implement database query
    // For now, return not found
    Err(ApiError::NotFound(format!(
        "Application not found: {package_id}"
    )))
}
