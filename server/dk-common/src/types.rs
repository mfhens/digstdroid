//! Common types for DK-AppStore.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for an application.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AppId(pub String);

impl AppId {
    /// Create a new `AppId` from a package identifier.
    ///
    /// # Example
    ///
    /// ```
    /// use dk_common::types::AppId;
    ///
    /// let id = AppId::new("dk.digst.mitid");
    /// ```
    #[must_use]
    pub fn new(package_id: impl Into<String>) -> Self {
        Self(package_id.into())
    }

    /// Returns the package identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AppId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Application metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    /// Unique identifier (UUID).
    pub id: Uuid,
    /// Package identifier (e.g., "dk.digst.mitid").
    pub package_id: AppId,
    /// Display name.
    pub name: String,
    /// Short description.
    pub summary: String,
    /// Full description.
    pub description: String,
    /// Current version code.
    pub version_code: i64,
    /// Current version name.
    pub version_name: String,
    /// When the app was added.
    pub created_at: DateTime<Utc>,
    /// When the app was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Application version information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVersion {
    /// Unique identifier (UUID).
    pub id: Uuid,
    /// Reference to the application.
    pub app_id: Uuid,
    /// Version code (Android versionCode).
    pub version_code: i64,
    /// Version name (Android versionName).
    pub version_name: String,
    /// SHA-256 hash of the APK.
    pub sha256: String,
    /// Size of the APK in bytes.
    pub size: i64,
    /// Minimum Android SDK version.
    pub min_sdk: i32,
    /// Target Android SDK version.
    pub target_sdk: i32,
    /// When this version was added.
    pub created_at: DateTime<Utc>,
}

/// Build status for an application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildStatus {
    /// Build is queued.
    Pending,
    /// Build is in progress.
    Building,
    /// Build completed successfully.
    Success,
    /// Build failed.
    Failed,
    /// Build was cancelled.
    Cancelled,
}

/// Security scan status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanStatus {
    /// Scan is pending.
    Pending,
    /// Scan is in progress.
    Scanning,
    /// Scan passed (no critical issues).
    Passed,
    /// Scan failed (critical issues found).
    Failed,
    /// Scan passed with warnings.
    Warning,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_id_display() {
        let id = AppId::new("dk.digst.mitid");
        assert_eq!(id.to_string(), "dk.digst.mitid");
        assert_eq!(id.as_str(), "dk.digst.mitid");
    }

    #[test]
    fn test_build_status_serde() {
        let status = BuildStatus::Success;
        let json = serde_json::to_string(&status).expect("serialize");
        assert_eq!(json, "\"success\"");
    }
}
