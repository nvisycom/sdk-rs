//! HTTP client for the Nvisy Server API.

mod config;
mod nvisy;

pub(crate) use self::config::NvisyBuilderError;
pub use self::config::{
    DEFAULT_BASE_URL, DEFAULT_MAX_RETRIES, DEFAULT_TIMEOUT, DEFAULT_USER_AGENT, NvisyBuilder,
};
pub use self::nvisy::Nvisy;
