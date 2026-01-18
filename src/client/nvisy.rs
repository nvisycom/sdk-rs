//! Nvisy API client implementation.
//!
//! This module contains the main [`NvisyClient`] struct and its implementation,
//! providing the core HTTP client functionality for interacting with the Nvisy API.

use std::fmt;
use std::sync::Arc;

use reqwest::multipart::Form;
use reqwest::{Client, Method, RequestBuilder, Response};

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
/// # fn example() -> Result<()> {
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
/// # fn example() -> Result<()> {
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
#[derive(Debug)]
pub(crate) struct NvisyClientInner {
    pub(crate) config: NvisyConfig,
    pub(crate) client: Client,
}

impl NvisyClient {
    /// Creates a new Nvisy API client with the given configuration.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(config), fields(api_key = %config.masked_api_key())))]
    pub fn new(config: NvisyConfig) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, "Creating Nvisy client");

        let client = if let Some(custom_client) = config.client() {
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

        let inner = Arc::new(NvisyClientInner { config, client });
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

    /// Parses the base URL and appends the given path.
    fn parse_url(&self, path: &str) -> Result<url::Url> {
        let mut url = url::Url::parse(self.inner.config.base_url())?;
        url.set_path(&format!("{}{}", url.path().trim_end_matches('/'), path));
        Ok(url)
    }

    /// Builds a URL with the given path and optional query parameters.
    fn build_url(&self, path: &str, params: &[(&str, &str)]) -> Result<url::Url> {
        let mut url = self.parse_url(path)?;

        if !params.is_empty() {
            url.query_pairs_mut().extend_pairs(params);
        }

        Ok(url)
    }

    /// Creates an HTTP request with the specified method.
    fn request(&self, method: Method, url: url::Url) -> RequestBuilder {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: TRACING_TARGET_CLIENT,
            url = %url,
            method = %method,
            "Creating HTTP request"
        );

        self.inner
            .client
            .request(method, url)
            .timeout(self.inner.config.timeout())
            .header(
                "Authorization",
                format!("Bearer {}", self.inner.config.api_key()),
            )
    }

    /// Sends a request and returns the response.
    #[allow(dead_code)]
    pub(crate) async fn send(&self, method: Method, path: &str) -> Result<Response> {
        let url = self.parse_url(path)?;
        let response = self.request(method, url).send().await?;
        Ok(response)
    }

    /// Sends a request with JSON body.
    #[allow(dead_code)]
    pub(crate) async fn send_json<T: serde::Serialize>(
        &self,
        method: Method,
        path: &str,
        data: &T,
    ) -> Result<Response> {
        let url = self.parse_url(path)?;
        let response = self.request(method, url).json(data).send().await?;
        Ok(response)
    }

    /// Sends a request with query parameters.
    #[allow(dead_code)]
    pub(crate) async fn send_with_params(
        &self,
        method: Method,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<Response> {
        let url = self.build_url(path, params)?;
        let response = self.request(method, url).send().await?;
        Ok(response)
    }

    /// Sends a request with multipart form data.
    #[allow(dead_code)]
    pub(crate) async fn send_multipart(
        &self,
        method: Method,
        path: &str,
        form: Form,
    ) -> Result<Response> {
        let url = self.parse_url(path)?;
        let response = self.request(method, url).multipart(form).send().await?;
        Ok(response)
    }

    /// Creates a request builder for custom query parameter building.
    /// Use this for complex query scenarios that need conditional parameters.
    #[allow(dead_code)]
    pub(crate) fn request_builder(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = self.parse_url(path)?;
        Ok(self.request(method, url))
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
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_client_creation() -> Result<()> {
        let client = NvisyClient::with_api_key("test-key")?;
        assert_eq!(client.config().api_key(), "test-key");
        assert_eq!(client.config().base_url(), "https://api.nvisy.com");
        Ok(())
    }

    #[test]
    fn test_client_creation_with_custom_config() -> Result<()> {
        let config = NvisyConfig::builder()
            .with_api_key("custom_key")
            .with_base_url("https://custom.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        let client = NvisyClient::new(config)?;

        assert_eq!(client.config().api_key(), "custom_key");
        assert_eq!(client.config().base_url(), "https://custom.api.com");
        assert_eq!(client.config().timeout(), Duration::from_secs(60));

        Ok(())
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
