//! Nvisy Runtime API client implementation.

use std::fmt;
use std::sync::Arc;

use reqwest::Method;
use reqwest::multipart::Form;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

use super::config::NvisyRtConfig;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CLIENT;
use crate::error::Result;

/// Main Nvisy Runtime API client.
///
/// The `NvisyRt` provides access to all Nvisy Runtime API endpoints.
/// It handles authentication, request/response serialization, and provides
/// a consistent async interface for all operations.
///
/// # Examples
///
/// ```no_run
/// use nvisy_rt_sdk::{NvisyRt, Result};
///
/// # fn example() -> Result<()> {
/// let client = NvisyRt::with_api_key("your-api-key")?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct NvisyRt {
    pub(crate) inner: Arc<NvisyRtInner>,
}

pub(crate) struct NvisyRtInner {
    pub(crate) config: NvisyRtConfig,
    pub(crate) client: ClientWithMiddleware,
}

impl NvisyRt {
    /// Creates a new client with the given configuration.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(config), fields(api_key = %config.masked_api_key())))]
    pub fn new(config: NvisyRtConfig) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, "Creating Nvisy Runtime client");

        let base_client = if let Some(custom_client) = config.client() {
            custom_client
        } else {
            reqwest::Client::builder()
                .timeout(config.timeout())
                .build()?
        };

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let builder =
            ClientBuilder::new(base_client).with(RetryTransientMiddleware::new_with_policy(
                retry_policy,
            ));

        #[cfg(feature = "tracing")]
        let builder = builder.with(reqwest_tracing::TracingMiddleware::default());

        let client = builder.build();

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: TRACING_TARGET_CLIENT,
            base_url = %config.base_url(),
            timeout = ?config.timeout(),
            api_key = %config.masked_api_key(),
            custom_client = config.client().is_some(),
            "Nvisy Runtime client created successfully"
        );

        let inner = Arc::new(NvisyRtInner { config, client });
        Ok(Self { inner })
    }

    /// Creates a new client with just an API key using default settings.
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        let config = NvisyRtConfig::builder().with_api_key(api_key).build()?;
        Self::new(config)
    }

    /// Creates a new configuration builder.
    pub fn builder() -> super::config::NvisyRtConfigBuilder {
        NvisyRtConfig::builder()
    }

    /// Returns a reference to the client configuration.
    pub fn config(&self) -> &NvisyRtConfig {
        &self.inner.config
    }

    fn parse_url(&self, path: &str) -> Result<url::Url> {
        let mut url = url::Url::parse(self.inner.config.base_url())?;
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
            .timeout(self.inner.config.timeout())
            .header(
                "Authorization",
                format!("Bearer {}", self.inner.config.api_key()),
            )
    }

    #[allow(dead_code)]
    pub(crate) async fn send(
        &self,
        method: Method,
        path: &str,
    ) -> Result<reqwest::Response> {
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

impl fmt::Debug for NvisyRt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NvisyRt")
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
        let client = NvisyRt::with_api_key("test-key")?;
        assert_eq!(client.config().api_key(), "test-key");
        assert_eq!(client.config().base_url(), "https://rt.nvisy.com");
        Ok(())
    }

    #[test]
    fn test_client_creation_with_custom_config() -> Result<()> {
        let config = NvisyRtConfig::builder()
            .with_api_key("custom_key")
            .with_base_url("https://custom.rt.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        let client = NvisyRt::new(config)?;

        assert_eq!(client.config().api_key(), "custom_key");
        assert_eq!(client.config().base_url(), "https://custom.rt.api.com");
        assert_eq!(client.config().timeout(), Duration::from_secs(60));

        Ok(())
    }

    #[test]
    fn test_client_clone() -> Result<()> {
        let client = NvisyRt::with_api_key("test-key")?;
        let cloned = client.clone();

        assert_eq!(client.config().api_key(), cloned.config().api_key());
        assert_eq!(client.config().base_url(), cloned.config().base_url());

        Ok(())
    }

    #[test]
    fn test_builder_convenience_method() -> Result<()> {
        let client = NvisyRt::builder().with_api_key("test_key").build_client()?;

        assert_eq!(client.config().api_key(), "test_key");

        Ok(())
    }
}
