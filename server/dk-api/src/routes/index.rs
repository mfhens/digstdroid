//! Repository index endpoint.

use axum::Json;
use serde::Serialize;

/// Repository index response.
///
/// Compatible with F-Droid index format.
#[derive(Serialize)]
pub struct IndexResponse {
    repo: RepoInfo,
    apps: Vec<serde_json::Value>,
    packages: std::collections::HashMap<String, Vec<serde_json::Value>>,
}

/// Repository information.
#[derive(Serialize)]
pub struct RepoInfo {
    name: String,
    description: String,
    timestamp: i64,
    version: i32,
}

/// Get the repository index.
///
/// GET /api/v1/index
///
/// Returns the repository index in a format compatible with F-Droid clients.
pub async fn get_index() -> Json<IndexResponse> {
    // TODO: Generate actual index from database
    // This should be cached and regenerated when apps change
    Json(IndexResponse {
        repo: RepoInfo {
            name: "DK-AppStore".to_string(),
            description: "Danish sovereign app distribution platform".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            version: 21, // F-Droid index version
        },
        apps: vec![],
        packages: std::collections::HashMap::new(),
    })
}
