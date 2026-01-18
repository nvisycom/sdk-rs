//! Error types for the Nvisy client.

use thiserror::Error;

/// Result type alias for Nvisy client operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when using the Nvisy client.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Configuration builder error.
    #[error("Configuration error: {0}")]
    Builder(#[from] crate::client::config::NvisyConfigBuilderError),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
