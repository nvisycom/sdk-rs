//! Nvisy API client implementation.
//!
//! This module contains the main [`NvisyClient`] struct and its implementation,
//! providing the core HTTP client functionality for interacting with the Nvisy API.

use std::fmt;
use std::sync::Arc;

use reqwest::{Client, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;

use super::config::NvisyConfig;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CLIENT;
use crate::error::Result;

/// Main Nvisy API client for interacting with all Nvisy services.
///
/// The `NvisyClient` provides access to all Nvisy API endpoints through specialized
/// service interfaces. It handles authentication, request/response serialization,
/// and provides a consistent async interface for all operations.
///
/// # Features
///
/// - **Thread-safe**: Safe to use across multiple threads
/// - **Cheap to clone**: Uses `Arc` internally for efficient cloning
/// - **Automatic authentication**: Handles API key authentication automatically
///
/// # Examples
///
/// ## Basic usage with API key
///
/// ```no_run
/// use nvisy_sdk::{NvisyClient, Result};
///
/// # async fn example() -> Result<()> {
/// let client = NvisyClient::with_api_key("your-api-key")?;
/// # Ok(())
/// # }
/// ```
///
/// ## Custom configuration with builder pattern
///
/// ```no_run
/// use nvisy_sdk::{NvisyConfig, NvisyClient, Result};
/// use std::time::Duration;
///
/// # async fn example() -> Result<()> {
/// let client = NvisyConfig::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://api.nvisy.com")
///     .with_timeout(Duration::from_secs(30))
///     .build_client()?;
/// # Ok(())
/// # }
/// ```
///
/// ## Multi-threaded usage
///
/// The client is cheap to clone (uses `Arc` internally):
///
/// ```no_run
/// use nvisy_sdk::{NvisyClient, Result};
/// use tokio::task;
///
/// # async fn example() -> Result<()> {
/// let client = NvisyClient::with_api_key("your-api-key")?;
///
/// let handles: Vec<_> = (0..3).map(|_| {
///     let client = client.clone();
///     task::spawn(async move {
///         // Use client here
///         Ok::<(), nvisy_sdk::Error>(())
///     })
/// }).collect();
///
/// for handle in handles {
///     handle.await.unwrap()?;
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct NvisyClient {
    pub(crate) inner: Arc<NvisyClientInner>,
}

/// Inner client state that is shared via Arc for cheap cloning.
pub(crate) struct NvisyClientInner {
    pub(crate) config: NvisyConfig,
    pub(crate) http: Client,
}

impl NvisyClient {
    /// Creates a new Nvisy API client with the given configuration.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(config), fields(api_key = %config.masked_api_key())))]
    pub fn new(config: NvisyConfig) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, "Creating Nvisy client");

        let http = if let Some(custom_client) = config.client() {
            custom_client
        } else {
            Client::builder().timeout(config.timeout()).build()?
        };

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: TRACING_TARGET_CLIENT,
            base_url = %config.base_url(),
            timeout = ?config.timeout(),
            api_key = %config.masked_api_key(),
            custom_client = config.client().is_some(),
            "Nvisy client created successfully"
        );

        let inner = Arc::new(NvisyClientInner { config, http });
        Ok(Self { inner })
    }

    /// Creates a new client with just an API key using default settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use nvisy_sdk::{NvisyClient, Result};
    /// # fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        let config = NvisyConfig::builder().with_api_key(api_key).build()?;
        Self::new(config)
    }

    /// Creates a new configuration builder for constructing a Nvisy client.
    ///
    /// This is a convenience method that returns a `NvisyConfigBuilder` for building
    /// a custom client configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use nvisy_sdk::{NvisyClient, Result};
    /// # use std::time::Duration;
    /// # fn example() -> Result<()> {
    /// let client = NvisyClient::builder()
    ///     .with_api_key("your-api-key")
    ///     .with_timeout(Duration::from_secs(60))
    ///     .build_client()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> super::config::NvisyConfigBuilder {
        NvisyConfig::builder()
    }

    /// Returns a reference to the client configuration.
    pub fn config(&self) -> &NvisyConfig {
        &self.inner.config
    }

    /// Builds a request with authentication headers.
    pub(crate) fn request(&self, method: Method, path: &str) -> RequestBuilder {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: TRACING_TARGET_CLIENT,
            path = %path,
            method = %method,
            "Creating HTTP request"
        );

        let url = format!("{}{}", self.inner.config.base_url(), path);
        self.inner
            .http
            .request(method, &url)
            .header(
                "Authorization",
                format!("Bearer {}", self.inner.config.api_key()),
            )
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
    }

    /// Builds a GET request.
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        self.request(Method::GET, path)
    }

    /// Builds a POST request.
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        self.request(Method::POST, path)
    }

    /// Builds a PUT request.
    pub(crate) fn put(&self, path: &str) -> RequestBuilder {
        self.request(Method::PUT, path)
    }

    /// Builds a DELETE request.
    pub(crate) fn delete_req(&self, path: &str) -> RequestBuilder {
        self.request(Method::DELETE, path)
    }

    /// Sends a request and parses the JSON response.
    pub(crate) async fn send<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Handles API response, checking for errors and parsing JSON.
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let body = response.error_for_status()?.json().await?;
        Ok(body)
    }

    /// Sends a request with a JSON body.
    pub(crate) async fn send_json<T, B>(&self, request: RequestBuilder, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        self.send(request.json(body)).await
    }

    /// Sends a DELETE request (no response body expected).
    pub(crate) async fn send_delete(&self, request: RequestBuilder) -> Result<()> {
        request.send().await?.error_for_status()?;
        Ok(())
    }

    /// Sends a request expecting raw bytes response.
    pub(crate) async fn send_bytes(&self, request: RequestBuilder) -> Result<Vec<u8>> {
        let response = request.send().await?.error_for_status()?;
        Ok(response.bytes().await?.to_vec())
    }

    /// Sends a request expecting a text response.
    pub(crate) async fn send_text(&self, request: RequestBuilder) -> Result<String> {
        let response = request.send().await?.error_for_status()?;
        Ok(response.text().await?)
    }
}

impl fmt::Debug for NvisyClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NvisyClient")
            .field("api_key", &self.inner.config.masked_api_key())
            .field("base_url", &self.inner.config.base_url())
            .field("timeout", &self.inner.config.timeout())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = NvisyClient::with_api_key("test-key");
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_config_access() {
        let client = NvisyClient::with_api_key("test-key").unwrap();
        assert_eq!(client.config().api_key(), "test-key");
    }

    #[test]
    fn test_client_clone() -> Result<()> {
        let client = NvisyClient::with_api_key("test-key")?;
        let cloned = client.clone();

        assert_eq!(client.config().api_key(), cloned.config().api_key());
        assert_eq!(client.config().base_url(), cloned.config().base_url());

        Ok(())
    }

    #[test]
    fn test_builder_convenience_method() -> Result<()> {
        let client = NvisyClient::builder()
            .with_api_key("test_key")
            .build_client()?;

        assert_eq!(client.config().api_key(), "test_key");

        Ok(())
    }

    #[test]
    fn test_debug_impl_masks_api_key() -> Result<()> {
        let client = NvisyClient::with_api_key("secret_api_key_12345")?;
        let debug_output = format!("{:?}", client);

        assert!(debug_output.contains("secr****"));
        assert!(!debug_output.contains("secret_api_key_12345"));

        Ok(())
    }
}
