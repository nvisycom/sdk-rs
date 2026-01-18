//! Error types for the Nvisy client.

use thiserror::Error;

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
    Builder(#[from] crate::client::NvisyConfigBuilderError),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for Nvisy API operations.
///
/// This is a convenience type alias for `std::result::Result<T, Error>` that is used
/// throughout the Nvisy SDK. All SDK methods that can fail return this Result type.
pub type Result<T, E = Error> = std::result::Result<T, E>;
