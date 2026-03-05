//! Error types for the Nvisy Runtime SDK.

use crate::client::NvisyRtConfigBuilderError;

/// Error type for Nvisy Runtime API operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP transport error from the underlying HTTP client.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(#[from] NvisyRtConfigBuilderError),

    /// URL parsing error.
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// API error.
    #[error("API error: {0}")]
    Api(String),
}

/// Result type for Nvisy Runtime API operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;
