//! Nvisy Runtime client configuration and builder.
//!
//! [`NvisyRt`]: crate::NvisyRt

use std::fmt;
use std::time::Duration;

use derive_builder::Builder;
use reqwest::Client;

use super::nvisy::NvisyRt;
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
    pub fn build_client(self) -> Result<NvisyRt> {
        let config = self.build()?;
        NvisyRt::new(config)
    }
}

impl NvisyRtConfig {
    /// Creates a new configuration builder.
    pub fn builder() -> NvisyRtConfigBuilder {
        NvisyRtConfigBuilder::default()
    }

    /// Creates a new client using this configuration.
    pub fn build_client(self) -> Result<NvisyRt> {
        NvisyRt::new(self)
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
        let config = NvisyRtConfig::builder().build()?;

        assert_eq!(config.base_url(), DEFAULT_BASE_URL);
        assert_eq!(config.timeout(), DEFAULT_TIMEOUT);

        Ok(())
    }

    #[test]
    fn test_config_builder_with_custom_values() -> Result<()> {
        let config = NvisyRtConfig::builder()
            .with_base_url("https://custom.rt.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        assert_eq!(config.base_url(), "https://custom.rt.api.com");
        assert_eq!(config.timeout(), Duration::from_secs(60));

        Ok(())
    }

    #[test]
    fn test_config_validation_invalid_base_url() {
        let result = NvisyRtConfig::builder()
            .with_base_url("not-a-url")
            .build();
        assert!(result.is_err());
    }
}
