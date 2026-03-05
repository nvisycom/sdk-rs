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

/// Internal configuration produced by the builder.
#[doc(hidden)]
#[derive(Clone, Builder)]
#[builder(
    name = "NvisyBuilder",
    pattern = "owned",
    setter(into, strip_option, prefix = "with"),
    build_fn(validate = "Self::validate", private, name = "build_config")
)]
pub struct NvisyOptions {
    /// API key for authentication with the Nvisy API.
    pub(crate) api_key: String,

    /// Base URL for the Nvisy API.
    #[builder(default = "Self::default_base_url()")]
    pub(crate) base_url: String,

    /// Timeout for HTTP requests.
    #[builder(default = "Self::default_timeout()")]
    pub(crate) timeout: Duration,

    /// Optional custom reqwest client.
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
