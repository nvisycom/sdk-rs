//! Nvisy Runtime client configuration and builder.

use std::fmt;
use std::time::Duration;

use derive_builder::Builder;
use reqwest::Client;

use super::rt::NvisyRtClient;
use crate::error::Result;

/// Default base URL for the Nvisy Runtime API.
pub const DEFAULT_BASE_URL: &str = "https://rt.nvisy.com";

/// Default request timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Configuration for the Nvisy Runtime API client.
#[derive(Clone, Builder)]
#[builder(
    name = "NvisyRtConfigBuilder",
    pattern = "owned",
    setter(into, strip_option, prefix = "with"),
    build_fn(validate = "Self::validate_config")
)]
pub struct NvisyRtConfig {
    /// API key for authentication.
    api_key: String,

    /// Base URL for the Nvisy Runtime API.
    #[builder(default = "Self::default_base_url()")]
    base_url: String,

    /// Timeout for HTTP requests.
    #[builder(default = "Self::default_timeout()")]
    timeout: Duration,

    /// Optional custom reqwest client.
    #[builder(default = "None")]
    client: Option<Client>,
}

impl NvisyRtConfigBuilder {
    fn default_base_url() -> String {
        DEFAULT_BASE_URL.to_string()
    }

    fn default_timeout() -> Duration {
        DEFAULT_TIMEOUT
    }

    fn validate_config(&self) -> std::result::Result<(), String> {
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

    /// Creates a client directly from the builder.
    pub fn build_client(self) -> Result<NvisyRtClient> {
        let config = self.build()?;
        NvisyRtClient::new(config)
    }
}

impl NvisyRtConfig {
    /// Creates a new configuration builder.
    pub fn builder() -> NvisyRtConfigBuilder {
        NvisyRtConfigBuilder::default()
    }

    /// Creates a new client using this configuration.
    pub fn build_client(self) -> Result<NvisyRtClient> {
        NvisyRtClient::new(self)
    }

    /// Returns the API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Returns a masked version of the API key for safe display/logging.
    pub fn masked_api_key(&self) -> String {
        if self.api_key.len() > 4 {
            format!("{}****", &self.api_key[..4])
        } else {
            "****".to_string()
        }
    }

    /// Returns the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the timeout duration.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Returns a clone of the custom reqwest client, if one was provided.
    pub(crate) fn client(&self) -> Option<Client> {
        self.client.clone()
    }
}

impl fmt::Debug for NvisyRtConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NvisyRtConfig")
            .field("api_key", &self.masked_api_key())
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() -> Result<()> {
        let config = NvisyRtConfig::builder().with_api_key("test_key").build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.base_url(), DEFAULT_BASE_URL);
        assert_eq!(config.timeout(), DEFAULT_TIMEOUT);

        Ok(())
    }

    #[test]
    fn test_config_builder_with_custom_values() -> Result<()> {
        let config = NvisyRtConfig::builder()
            .with_api_key("test_key")
            .with_base_url("https://custom.rt.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.base_url(), "https://custom.rt.api.com");
        assert_eq!(config.timeout(), Duration::from_secs(60));

        Ok(())
    }

    #[test]
    fn test_config_validation_empty_api_key() {
        let result = NvisyRtConfig::builder().with_api_key("").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_invalid_base_url() {
        let result = NvisyRtConfig::builder()
            .with_api_key("test_key")
            .with_base_url("not-a-url")
            .build();
        assert!(result.is_err());
    }
}
