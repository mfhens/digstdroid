//! Configuration management for DK-AppStore.

use serde::Deserialize;

/// Application configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Database configuration.
    pub database: DatabaseConfig,
    /// Redis configuration.
    pub redis: RedisConfig,
    /// API server configuration.
    pub api: ApiConfig,
}

/// Database configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL.
    pub url: String,
    /// Maximum number of connections in the pool.
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

/// Redis configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL.
    pub url: String,
}

/// API server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    /// Host to bind to.
    #[serde(default = "default_host")]
    pub host: String,
    /// Port to listen on.
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_max_connections() -> u32 {
    10
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
}

impl Config {
    /// Load configuration from environment variables and optional config file.
    ///
    /// # Errors
    ///
    /// Returns an error if required configuration is missing or invalid.
    pub fn load() -> Result<Self, config::ConfigError> {
        // Load .env file if present (ignore errors)
        let _ = dotenvy::dotenv();

        config::Config::builder()
            .add_source(config::Environment::with_prefix("DK_APPSTORE").separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        assert_eq!(default_max_connections(), 10);
        assert_eq!(default_host(), "127.0.0.1");
        assert_eq!(default_port(), 8080);
    }
}
