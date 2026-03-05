//! Nvisy Runtime API client implementation.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use reqwest::Method;
use reqwest::multipart::Form;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;

use super::config::{NvisyRtBuilder, NvisyRtOptions};
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_CLIENT;
use crate::error::Result;

/// Main Nvisy Runtime API client.
///
/// [`NvisyRt`] provides access to all Nvisy Runtime API endpoints for
/// direct multimodal redaction. It handles request/response serialization,
/// automatic retries with exponential backoff, and optional [`tracing`]
/// instrumentation.
///
/// # Features
///
/// - **Thread-safe**: safe to share across threads and tasks
/// - **Cheap to clone**: uses [`Arc`] internally
/// - **Automatic retries**: retries transient failures with exponential backoff
/// - **No auth required**: connects directly to a runtime instance
///
/// # Examples
///
/// ```no_run
/// use nvisy_rt_sdk::{NvisyRt, Result};
///
/// # fn example() -> Result<()> {
/// let client = NvisyRt::new()?;
/// # Ok(())
/// # }
/// ```
///
/// ## Custom configuration
///
/// ```no_run
/// use nvisy_rt_sdk::{NvisyRt, Result};
/// use std::time::Duration;
///
/// # fn example() -> Result<()> {
/// let client = NvisyRt::builder()
///     .with_base_url("http://runtime.local:8080")
///     .with_timeout(Duration::from_secs(60))
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// [`Arc`]: std::sync::Arc
/// [`tracing`]: https://docs.rs/tracing
/// [`NvisyRt`]: crate::NvisyRt
#[derive(Clone)]
pub struct NvisyRt {
    pub(crate) inner: Arc<NvisyRtInner>,
}

pub(crate) struct NvisyRtInner {
    pub(crate) base_url: String,
    pub(crate) timeout: Duration,
    pub(crate) client: ClientWithMiddleware,
}

impl NvisyRt {
    /// Creates a new client with default settings.
    ///
    /// Connects to [`DEFAULT_BASE_URL`] with a [`DEFAULT_TIMEOUT`] of 30 seconds.
    ///
    /// [`DEFAULT_BASE_URL`]: crate::DEFAULT_BASE_URL
    /// [`DEFAULT_TIMEOUT`]: crate::DEFAULT_TIMEOUT
    pub fn new() -> Result<Self> {
        NvisyRtBuilder::default().build()
    }

    /// Creates a new builder for constructing a client with custom settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use nvisy_rt_sdk::{NvisyRt, Result};
    /// # use std::time::Duration;
    /// # fn example() -> Result<()> {
    /// let client = NvisyRt::builder()
    ///     .with_base_url("http://runtime.local:8080")
    ///     .with_timeout(Duration::from_secs(60))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> NvisyRtBuilder {
        NvisyRtBuilder::default()
    }

    /// Creates a client from validated options (called by the builder).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(options)))]
    pub(crate) fn from_options(options: NvisyRtOptions) -> Result<Self> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, "Creating Nvisy Runtime client");

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
            "Nvisy Runtime client created"
        );

        let inner = Arc::new(NvisyRtInner {
            base_url: options.base_url,
            timeout: options.timeout,
            client,
        });
        Ok(Self { inner })
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
            %url,
            %method,
            "Building request"
        );

        self.inner
            .client
            .request(method, url)
            .timeout(self.inner.timeout)
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub(crate) async fn send_with_params(
        &self,
        method: Method,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<reqwest::Response> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, %method, path, "Sending request with params");

        let url = self.build_url(path, params)?;
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

    #[allow(dead_code)]
    pub(crate) async fn send_multipart(
        &self,
        method: Method,
        path: &str,
        form: Form,
    ) -> Result<reqwest::Response> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_CLIENT, %method, path, "Sending multipart request");

        let url = self.parse_url(path)?;
        let response = self.request(method, url).multipart(form).send().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_CLIENT,
            status = response.status().as_u16(),
            path,
            "Response received"
        );

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
        let client = NvisyRt::new()?;
        assert_eq!(client.base_url(), "http://localhost:8080");
        Ok(())
    }

    #[test]
    fn test_client_creation_with_custom_config() -> Result<()> {
        let client = NvisyRt::builder()
            .with_base_url("https://custom.rt.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        assert_eq!(client.base_url(), "https://custom.rt.api.com");
        assert_eq!(client.timeout(), Duration::from_secs(60));

        Ok(())
    }
}
