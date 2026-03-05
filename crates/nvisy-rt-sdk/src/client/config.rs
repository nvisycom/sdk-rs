//! Nvisy Runtime client builder.

use std::time::Duration;

use derive_builder::Builder;
use reqwest::Client;

use super::nvisy::NvisyRt;
use crate::error::Result;

/// Default base URL for the Nvisy Runtime API.
pub const DEFAULT_BASE_URL: &str = "http://localhost:8080";

/// Default request timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default maximum number of retries for transient failures.
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Default User-Agent header value.
pub const DEFAULT_USER_AGENT: &str = concat!(
    "NvisyRtSDK/",
    env!("CARGO_PKG_VERSION"),
    " (Rust; reqwest/0.13)"
);

/// Internal configuration produced by the builder.
#[doc(hidden)]
#[derive(Clone, Builder)]
#[builder(
    name = "NvisyRtBuilder",
    pattern = "owned",
    setter(into, strip_option, prefix = "with"),
    build_fn(validate = "Self::validate", private, name = "build_config")
)]
#[builder_struct_attr(doc = "Builder for configuring and creating a [`NvisyRt`] client.\n\n[`NvisyRt`]: crate::NvisyRt")]
pub struct NvisyRtOptions {
    /// Base URL for the Nvisy Runtime API.
    #[builder(default = "Self::default_base_url()")]
    pub(crate) base_url: String,

    /// Timeout for HTTP requests.
    #[builder(default = "Self::default_timeout()")]
    pub(crate) timeout: Duration,

    /// Maximum number of retries for transient failures.
    #[builder(default = "DEFAULT_MAX_RETRIES")]
    pub(crate) max_retries: u32,

    /// User-Agent header sent with every request.
    #[builder(default = "Self::default_user_agent()")]
    pub(crate) user_agent: String,

    /// Optional custom reqwest client.
    #[builder(default = "None")]
    pub(crate) client: Option<Client>,
}

impl NvisyRtBuilder {
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

    /// Builds the Nvisy Runtime client.
    pub fn build(self) -> Result<NvisyRt> {
        let options = self.build_config()?;
        NvisyRt::from_options(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_invalid_base_url() {
        let result = NvisyRtBuilder::default().with_base_url("not-a-url").build();
        assert!(result.is_err());
    }
}
