//! Error types for the Nvisy Runtime SDK.

use crate::client::NvisyRtBuilderError;

/// Error type for Nvisy Runtime API operations.
///
/// This enum represents all possible errors that can occur when using the
/// Nvisy Runtime SDK, from HTTP transport errors to API-specific failures
/// and configuration issues.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP middleware error from the underlying HTTP client.
    ///
    /// This includes network connectivity issues, DNS resolution failures,
    /// timeout errors, retry exhaustion, and other transport-layer problems.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest_middleware::Error),

    /// HTTP transport error from the underlying HTTP client.
    ///
    /// This covers errors from response status checks and other direct
    /// reqwest operations.
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    ///
    /// This occurs when the SDK fails to parse API responses or serialize
    /// request payloads to/from JSON.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Configuration error.
    ///
    /// This occurs when configuration parameters are invalid or when using
    /// the configuration builder and validation fails during the build process.
    #[error("Configuration error: {0}")]
    Config(#[from] NvisyRtBuilderError),

    /// URL parsing error.
    ///
    /// This occurs when a provided URL string is invalid or cannot be parsed.
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// I/O error.
    ///
    /// This occurs during file operations or other I/O tasks.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// API error.
    ///
    /// This occurs when the API returns an unexpected response format
    /// or missing data that was expected.
    #[error("API error: {0}")]
    Api(String),
}

/// Result type for Nvisy Runtime API operations.
///
/// Convenience alias for `std::result::Result<T, Error>` used throughout the
/// Nvisy Runtime SDK.
pub type Result<T, E = Error> = std::result::Result<T, E>;
