//! Nvisy Server client builder.

use std::time::Duration;

use derive_builder::Builder;
use reqwest::Client;

use super::nvisy::Nvisy;
use crate::error::Result;

/// Default base URL for the Nvisy API.
pub const DEFAULT_BASE_URL: &str = "https://api.nvisy.com";

/// Default request timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default maximum number of retries for transient failures.
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Default User-Agent header value.
pub const DEFAULT_USER_AGENT: &str = concat!(
    "NvisySDK/",
    env!("CARGO_PKG_VERSION"),
    " (Rust; reqwest/0.13)"
);

/// Internal configuration produced by the builder.
#[doc(hidden)]
#[derive(Clone, Builder)]
#[builder(
    name = "NvisyBuilder",
    pattern = "owned",
    setter(into, strip_option, prefix = "with"),
    build_fn(validate = "Self::validate", private, name = "build_config")
)]
#[builder_struct_attr(doc = "Builder for configuring and creating a [`Nvisy`] client.\n\n[`Nvisy`]: crate::Nvisy")]
pub struct NvisyOptions {
    /// API key for authentication with the Nvisy API.
    ///
    /// Required. The key is sent as a `Bearer` token on every request.
    pub(crate) api_key: String,

    /// Base URL for the Nvisy API.
    ///
    /// Defaults to [`DEFAULT_BASE_URL`].
    #[builder(default = "Self::default_base_url()")]
    pub(crate) base_url: String,

    /// Timeout for HTTP requests.
    ///
    /// Defaults to [`DEFAULT_TIMEOUT`].
    #[builder(default = "Self::default_timeout()")]
    pub(crate) timeout: Duration,

    /// Maximum number of retries for transient failures.
    ///
    /// Set to `0` to disable retries. Defaults to [`DEFAULT_MAX_RETRIES`].
    #[builder(default = "DEFAULT_MAX_RETRIES")]
    pub(crate) max_retries: u32,

    /// User-Agent header sent with every request.
    ///
    /// Defaults to [`DEFAULT_USER_AGENT`].
    #[builder(default = "Self::default_user_agent()")]
    pub(crate) user_agent: String,

    /// Custom [`reqwest::Client`] to use for HTTP requests.
    ///
    /// When set, `timeout` and `user_agent` must be configured on
    /// the provided client directly.
    #[builder(default = "None")]
    pub(crate) client: Option<Client>,
}

impl NvisyBuilder {
    fn default_base_url() -> String {
        DEFAULT_BASE_URL.to_string()
    }

    fn default_timeout() -> Duration {
        DEFAULT_TIMEOUT
    }

    fn default_user_agent() -> String {
        DEFAULT_USER_AGENT.to_string()
    }

    fn validate(&self) -> std::result::Result<(), String> {
        if let Some(ref api_key) = self.api_key
            && api_key.trim().is_empty()
        {
            return Err("API key cannot be empty".to_string());
        }

        if let Some(ref base_url) = self.base_url
            && !base_url.starts_with("http://")
            && !base_url.starts_with("https://")
        {
            return Err("Base URL must start with http:// or https://".to_string());
        }

        if let Some(timeout) = self.timeout {
            if timeout.is_zero() {
                return Err("Timeout must be greater than 0".to_string());
            }
            if timeout > Duration::from_secs(300) {
                return Err("Timeout cannot exceed 300 seconds (5 minutes)".to_string());
            }
        }

        Ok(())
    }

    /// Sets the timeout in seconds.
    pub fn with_timeout_secs(self, secs: u64) -> Self {
        self.with_timeout(Duration::from_secs(secs))
    }

    /// Builds the Nvisy API client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use nvisy_sdk::Nvisy;
    /// let client = Nvisy::builder()
    ///     .with_api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Nvisy> {
        let options = self.build_config()?;
        Nvisy::from_options(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_empty_api_key() {
        let result = NvisyBuilder::default().with_api_key("").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_invalid_base_url() {
        let result = NvisyBuilder::default()
            .with_api_key("test_key")
            .with_base_url("not-a-url")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_zero_timeout() {
        let result = NvisyBuilder::default()
            .with_api_key("test_key")
            .with_timeout(Duration::from_secs(0))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_excessive_timeout() {
        let result = NvisyBuilder::default()
            .with_api_key("test_key")
            .with_timeout(Duration::from_secs(400))
            .build();
        assert!(result.is_err());
    }
}
