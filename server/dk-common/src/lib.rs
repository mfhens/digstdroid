//! DK-AppStore Common Library
//!
//! Shared types, utilities, and configuration for DK-AppStore components.

pub mod config;
pub mod error;
pub mod types;

pub use config::Config;
pub use error::{Error, Result};
