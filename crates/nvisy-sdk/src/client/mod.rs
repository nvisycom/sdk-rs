//! HTTP client for the Nvisy Server API.

mod config;
mod nvisy;

pub(crate) use config::NvisyBuilderError;
pub use config::{DEFAULT_BASE_URL, DEFAULT_TIMEOUT, NvisyBuilder};
pub use nvisy::Nvisy;
