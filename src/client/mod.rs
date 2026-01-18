//! HTTP client for the Nvisy API.

mod config;
mod nvisy;

pub(crate) use config::NvisyConfigBuilderError;
pub use config::{DEFAULT_BASE_URL, DEFAULT_TIMEOUT, NvisyConfig, NvisyConfigBuilder};
pub use nvisy::NvisyClient;
