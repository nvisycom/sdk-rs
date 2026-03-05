//! Nvisy Server API client implementation.
//!
//! This module contains the main [`Nvisy`] client and its implementation,
//! providing the core HTTP client for interacting with the Nvisy Server API.
//!
//! [`Nvisy`]: crate::Nvisy

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use reqwest::Method;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use url::Url;

use super::config::{NvisyBuilder, NvisyOptions};
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CLIENT;
use crate::error::Result;

/// Main Nvisy Server API client.
///
/// [`Nvisy`] provides access to all Nvisy Server API endpoints through
/// specialized service traits. It handles authentication, request/response
/// serialization, automatic retries with exponential backoff, and optional
/// [`tracing`] instrumentation.
///
/// # Features
///
/// - **Thread-safe**: safe to share across threads and tasks
/// - **Cheap to clone**: uses [`Arc`] internally
/// - **Automatic authentication**: injects API key on every request
/// - **Automatic retries**: retries transient failures with exponential backoff
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
/// ## Custom configuration
///
/// ```no_run
/// use nvisy_sdk::{Nvisy, Result};
/// use std::time::Duration;
///
/// # fn example() -> Result<()> {
/// let client = Nvisy::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://api.nvisy.com")
///     .with_timeout(Duration::from_secs(60))
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// [`Arc`]: std::sync::Arc
/// [`tracing`]: https://docs.rs/tracing
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
    /// Creates a new client with an API key and default settings.
    ///
    /// Connects to [`DEFAULT_BASE_URL`] with a [`DEFAULT_TIMEOUT`] of 30 seconds.
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
    ///
    /// [`DEFAULT_BASE_URL`]: crate::DEFAULT_BASE_URL
    /// [`DEFAULT_TIMEOUT`]: crate::DEFAULT_TIMEOUT
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
                .user_agent(&options.user_agent)
                .build()?
        };

        let retry_policy =
            ExponentialBackoff::builder().build_with_max_retries(options.max_retries);
        let builder = ClientBuilder::new(base_client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy));

        #[cfg(feature = "tracing")]
        let builder = builder.with(reqwest_tracing::TracingMiddleware::default());

        let client = builder.build();

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: TRACING_TARGET_CLIENT,
            base_url = %options.base_url,
            timeout_secs = options.timeout.as_secs(),
            api_key = %Self::mask_key(&options.api_key),
            "Nvisy client created"
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

    /// Returns a masked version of the API key for safe display and logging.
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

    fn parse_url(&self, path: &str) -> Result<Url> {
        let mut url = Url::parse(&self.inner.base_url)?;
        url.set_path(&format!("{}{}", url.path().trim_end_matches('/'), path));
        Ok(url)
    }

    fn request(&self, method: Method, url: Url) -> RequestBuilder {
        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: TRACING_TARGET_CLIENT,
            %url,
            %method,
            "Building request"
        );

        self.inner
            .client
            .request(method, url)
            .timeout(self.inner.timeout)
            .header("Authorization", format!("Bearer {}", self.inner.api_key))
    }

    pub(crate) async fn send(&self, method: Method, path: &str) -> Result<reqwest::Response> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, %method, path, "Sending request");

        let url = self.parse_url(path)?;
        let response = self.request(method, url).send().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_CLIENT,
            status = response.status().as_u16(),
            path,
            "Response received"
        );

        Ok(response)
    }

    pub(crate) async fn send_json<T: serde::Serialize>(
        &self,
        method: Method,
        path: &str,
        data: &T,
    ) -> Result<reqwest::Response> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, %method, path, "Sending JSON request");

        let url = self.parse_url(path)?;
        let response = self.request(method, url).json(data).send().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_CLIENT,
            status = response.status().as_u16(),
            path,
            "Response received"
        );

        Ok(response)
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
