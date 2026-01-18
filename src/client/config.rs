//! Nvisy client configuration and builder.
//!
//! This module provides the configuration types and builder pattern for creating
//! and customizing [`NvisyClient`] instances.

use std::fmt;
use std::time::Duration;

use derive_builder::Builder;
use reqwest::Client;

use super::nvisy::NvisyClient;
use crate::error::Result;

/// Default base URL for the Nvisy API.
pub const DEFAULT_BASE_URL: &str = "https://api.nvisy.com";

/// Default request timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Configuration for the Nvisy API client.
///
/// This struct holds all the necessary configuration parameters for creating and using
/// a Nvisy API client, including authentication credentials, API endpoint information,
/// and HTTP client settings.
///
/// # Examples
///
/// Creating a config with API key:
/// ```no_run
/// use nvisy_studio_client::NvisyConfig;
///
/// let config = NvisyConfig::builder()
///     .with_api_key("your-api-key")
///     .build()
///     .unwrap();
/// ```
///
/// Creating a config with custom settings:
/// ```no_run
/// use std::time::Duration;
/// use nvisy_studio_client::NvisyConfig;
///
/// let config = NvisyConfig::builder()
///     .with_api_key("your-api-key")
///     .with_base_url("https://custom.api.nvisy.com")
///     .with_timeout(Duration::from_secs(60))
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Builder)]
#[builder(
    name = "NvisyConfigBuilder",
    pattern = "owned",
    setter(into, strip_option, prefix = "with"),
    build_fn(validate = "Self::validate_config")
)]
pub struct NvisyConfig {
    /// API key for authentication with the Nvisy API.
    api_key: String,

    /// Base URL for the Nvisy API.
    ///
    /// Defaults to the official Nvisy API endpoint.
    #[builder(default = "Self::default_base_url()")]
    base_url: String,

    /// Timeout for HTTP requests.
    ///
    /// Controls how long the client will wait for API responses before timing out.
    #[builder(default = "Self::default_timeout()")]
    timeout: Duration,

    /// Optional custom reqwest client.
    ///
    /// If provided, this client will be used instead of creating a new one.
    /// This allows for custom configuration of the HTTP client.
    #[builder(default = "None")]
    client: Option<Client>,
}

impl NvisyConfigBuilder {
    /// Returns the default base URL for the Nvisy API.
    fn default_base_url() -> String {
        DEFAULT_BASE_URL.to_string()
    }

    /// Returns the default timeout.
    fn default_timeout() -> Duration {
        DEFAULT_TIMEOUT
    }

    /// Validates the configuration before building.
    fn validate_config(&self) -> std::result::Result<(), String> {
        // Validate API key is not empty
        if let Some(ref api_key) = self.api_key
            && api_key.trim().is_empty()
        {
            return Err("API key cannot be empty".to_string());
        }

        // Validate base URL
        if let Some(ref base_url) = self.base_url
            && !base_url.starts_with("http://")
            && !base_url.starts_with("https://")
        {
            return Err("Base URL must start with http:// or https://".to_string());
        }

        // Validate timeout is reasonable
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

    /// Creates a Nvisy API client directly from the builder.
    ///
    /// This is a convenience method that builds the configuration and
    /// creates a client in one step.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use nvisy_studio_client::NvisyConfig;
    /// let client = NvisyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build_client()
    ///     .unwrap();
    /// ```
    pub fn build_client(self) -> Result<NvisyClient> {
        let config = self.build()?;
        NvisyClient::new(config)
    }
}

impl NvisyConfig {
    /// Creates a new configuration builder.
    ///
    /// This is the recommended way to construct a `NvisyConfig`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use nvisy_studio_client::NvisyConfig;
    /// let config = NvisyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> NvisyConfigBuilder {
        NvisyConfigBuilder::default()
    }

    /// Creates a new Nvisy API client using this configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use nvisy_studio_client::NvisyConfig;
    /// let config = NvisyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .build()
    ///     .unwrap();
    ///
    /// let client = config.build_client().unwrap();
    /// ```
    pub fn build_client(self) -> Result<NvisyClient> {
        NvisyClient::new(self)
    }

    /// Returns the API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Returns a masked version of the API key for safe display/logging.
    ///
    /// Shows the first 4 characters followed by "****", or just "****"
    /// if the key is shorter than 4 characters.
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

impl fmt::Debug for NvisyConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NvisyConfig")
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
        let config = NvisyConfig::builder().with_api_key("test_key").build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.base_url(), DEFAULT_BASE_URL);
        assert_eq!(config.timeout(), DEFAULT_TIMEOUT);

        Ok(())
    }

    #[test]
    fn test_config_builder_with_custom_values() -> Result<()> {
        let config = NvisyConfig::builder()
            .with_api_key("test_key")
            .with_base_url("https://custom.api.com")
            .with_timeout(Duration::from_secs(60))
            .build()?;

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.base_url(), "https://custom.api.com");
        assert_eq!(config.timeout(), Duration::from_secs(60));

        Ok(())
    }

    #[test]
    fn test_config_builder_timeout_secs() -> Result<()> {
        let config = NvisyConfig::builder()
            .with_api_key("test_key")
            .with_timeout_secs(45)
            .build()?;

        assert_eq!(config.timeout(), Duration::from_secs(45));

        Ok(())
    }

    #[test]
    fn test_config_validation_empty_api_key() {
        let result = NvisyConfig::builder().with_api_key("").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_invalid_base_url() {
        let result = NvisyConfig::builder()
            .with_api_key("test_key")
            .with_base_url("not-a-url")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_zero_timeout() {
        let result = NvisyConfig::builder()
            .with_api_key("test_key")
            .with_timeout(Duration::from_secs(0))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_excessive_timeout() {
        let result = NvisyConfig::builder()
            .with_api_key("test_key")
            .with_timeout(Duration::from_secs(400))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_masked_api_key() -> Result<()> {
        let config = NvisyConfig::builder()
            .with_api_key("test_key_12345")
            .build()?;

        assert_eq!(config.masked_api_key(), "test****");

        Ok(())
    }

    #[test]
    fn test_masked_api_key_short() -> Result<()> {
        let config = NvisyConfig::builder().with_api_key("abc").build()?;

        assert_eq!(config.masked_api_key(), "****");

        Ok(())
    }

    #[test]
    fn test_debug_masks_api_key() -> Result<()> {
        let config = NvisyConfig::builder()
            .with_api_key("secret_api_key_12345")
            .build()?;

        let debug_output = format!("{:?}", config);
        assert!(debug_output.contains("secr****"));
        assert!(!debug_output.contains("secret_api_key_12345"));

        Ok(())
    }
}
