//! Nvisy API client implementation.
//!
//! This module contains the main [`Nvisy`] struct and its implementation,
//! providing the core HTTP client functionality for interacting with the Nvisy API.
//!
//! [`Nvisy`]: crate::Nvisy

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use reqwest::Method;
use reqwest::multipart::Form;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;

use super::config::{NvisyBuilder, NvisyOptions};
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CLIENT;
use crate::error::Result;

/// Main Nvisy API client for interacting with all Nvisy services.
///
/// The [`Nvisy`] provides access to all Nvisy API endpoints through specialized
/// service interfaces. It handles authentication, request/response serialization,
/// and provides a consistent async interface for all operations.
///
/// # Features
///
/// - **Thread-safe**: Safe to use across multiple threads
/// - **Cheap to clone**: Uses `Arc` internally for efficient cloning
/// - **Automatic authentication**: Handles API key authentication automatically
/// - **Automatic retries**: Retries transient failures with exponential backoff
///
/// # Examples
///
/// ## Basic usage with API key
///
/// ```no_run
/// use nvisy_sdk::{Nvisy, Result};
///
/// # fn example() -> Result<()> {
/// let client = Nvisy::with_api_key("your-api-key")?;
/// # Ok(())
/// # }
/// ```
///
/// ## Custom configuration with builder pattern
///
/// ```no_run
/// use nvisy_sdk::{Nvisy, Result};
/// use std::time::Duration;
///
/// # fn example() -> Result<()> {
/// let client = Nvisy::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://api.nvisy.com")
///     .with_timeout(Duration::from_secs(30))
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// ## Multi-threaded usage
///
/// The client is cheap to clone (uses `Arc` internally):
///
/// ```no_run
/// use nvisy_sdk::{Nvisy, Result};
/// use tokio::task;
///
/// # async fn example() -> Result<()> {
/// let client = Nvisy::with_api_key("your-api-key")?;
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
///
/// [`Nvisy`]: crate::Nvisy
#[derive(Clone)]
pub struct Nvisy {
    pub(crate) inner: Arc<NvisyInner>,
}

pub(crate) struct NvisyInner {
    pub(crate) api_key: String,
    pub(crate) base_url: String,
    pub(crate) timeout: Duration,
    pub(crate) client: ClientWithMiddleware,
}

impl Nvisy {
    /// Creates a new client with just an API key using default settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use nvisy_sdk::{Nvisy, Result};
    /// # fn example() -> Result<()> {
    /// let client = Nvisy::with_api_key("your-api-key")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        NvisyBuilder::default().with_api_key(api_key).build()
    }

    /// Creates a new builder for constructing a client with custom settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use nvisy_sdk::{Nvisy, Result};
    /// # use std::time::Duration;
    /// # fn example() -> Result<()> {
    /// let client = Nvisy::builder()
    ///     .with_api_key("your-api-key")
    ///     .with_timeout(Duration::from_secs(60))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> NvisyBuilder {
        NvisyBuilder::default()
    }

    /// Creates a client from validated options (called by the builder).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(options), fields(api_key = %Self::mask_key(&options.api_key))))]
    pub(crate) fn from_options(options: NvisyOptions) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, "Creating Nvisy client");

        let base_client = if let Some(custom_client) = options.client {
            custom_client
        } else {
            reqwest::Client::builder()
                .timeout(options.timeout)
                .build()?
        };

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let builder = ClientBuilder::new(base_client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy));

        #[cfg(feature = "tracing")]
        let builder = builder.with(reqwest_tracing::TracingMiddleware::default());

        let client = builder.build();

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: TRACING_TARGET_CLIENT,
            base_url = %options.base_url,
            timeout = ?options.timeout,
            api_key = %Self::mask_key(&options.api_key),
            "Nvisy client created successfully"
        );

        let inner = Arc::new(NvisyInner {
            api_key: options.api_key,
            base_url: options.base_url,
            timeout: options.timeout,
            client,
        });
        Ok(Self { inner })
    }

    /// Returns the API key.
    pub fn api_key(&self) -> &str {
        &self.inner.api_key
    }

    /// Returns a masked version of the API key for safe display/logging.
    pub fn masked_api_key(&self) -> String {
        Self::mask_key(&self.inner.api_key)
    }

    fn mask_key(key: &str) -> String {
        if key.len() > 4 {
            format!("{}****", &key[..4])
        } else {
            "****".to_string()
        }
    }

    /// Returns the base URL.
    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    /// Returns the timeout duration.
    pub fn timeout(&self) -> Duration {
        self.inner.timeout
    }

    fn parse_url(&self, path: &str) -> Result<url::Url> {
        let mut url = url::Url::parse(&self.inner.base_url)?;
        url.set_path(&format!("{}{}", url.path().trim_end_matches('/'), path));
        Ok(url)
    }

    fn build_url(&self, path: &str, params: &[(&str, &str)]) -> Result<url::Url> {
        let mut url = self.parse_url(path)?;

        if !params.is_empty() {
            url.query_pairs_mut().extend_pairs(params);
        }

        Ok(url)
    }

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
            .timeout(self.inner.timeout)
            .header(
                "Authorization",
                format!("Bearer {}", self.inner.api_key),
            )
    }

    #[allow(dead_code)]
    pub(crate) async fn send(&self, method: Method, path: &str) -> Result<reqwest::Response> {
        let url = self.parse_url(path)?;
        let response = self.request(method, url).send().await?;
        Ok(response)
    }

    #[allow(dead_code)]
    pub(crate) async fn send_json<T: serde::Serialize>(
        &self,
        method: Method,
        path: &str,
        data: &T,
    ) -> Result<reqwest::Response> {
        let url = self.parse_url(path)?;
        let response = self.request(method, url).json(data).send().await?;
        Ok(response)
    }

    #[allow(dead_code)]
    pub(crate) async fn send_with_params(
        &self,
        method: Method,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<reqwest::Response> {
        let url = self.build_url(path, params)?;
        let response = self.request(method, url).send().await?;
        Ok(response)
    }

    #[allow(dead_code)]
    pub(crate) async fn send_multipart(
        &self,
        method: Method,
        path: &str,
        form: Form,
    ) -> Result<reqwest::Response> {
        let url = self.parse_url(path)?;
        let response = self.request(method, url).multipart(form).send().await?;
        Ok(response)
    }

    #[allow(dead_code)]
    pub(crate) fn request_builder(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = self.parse_url(path)?;
        Ok(self.request(method, url))
    }
}

impl fmt::Debug for Nvisy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Nvisy")
            .field("api_key", &self.masked_api_key())
            .field("base_url", &self.inner.base_url)
            .field("timeout", &self.inner.timeout)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_client_creation() -> Result<()> {
        let client = Nvisy::with_api_key("test-key")?;
        assert_eq!(client.api_key(), "test-key");
        assert_eq!(client.base_url(), "https://api.nvisy.com");
        Ok(())
    }

    #[test]
    fn test_client_creation_with_builder() -> Result<()> {
        let client = Nvisy::builder()
            .with_api_key("custom_key")
            .with_base_url("https://custom.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        assert_eq!(client.api_key(), "custom_key");
        assert_eq!(client.base_url(), "https://custom.api.com");
        assert_eq!(client.timeout(), Duration::from_secs(60));

        Ok(())
    }

    #[test]
    fn test_debug_impl_masks_api_key() -> Result<()> {
        let client = Nvisy::with_api_key("secret_api_key_12345")?;
        let debug_output = format!("{:?}", client);

        assert!(debug_output.contains("secr****"));
        assert!(!debug_output.contains("secret_api_key_12345"));

        Ok(())
    }
}
