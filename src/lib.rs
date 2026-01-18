#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
mod error;

pub mod model;
pub mod service;

#[doc(hidden)]
pub mod prelude;

/// Tracing target for client operations.
#[cfg(feature = "tracing")]
pub(crate) const TRACING_TARGET_CLIENT: &str = "nvisy_sdk::client";

// Re-export client types
pub use client::{DEFAULT_BASE_URL, DEFAULT_TIMEOUT, NvisyClient, NvisyConfig, NvisyConfigBuilder};

// Re-export error types
pub use error::{Error, Result};
