#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "rustls-tls", feature = "native-tls"))]
compile_error!("Features `rustls-tls` and `native-tls` are mutually exclusive: pick one");

#[cfg(not(any(feature = "rustls-tls", feature = "native-tls")))]
compile_error!("Either `rustls-tls` or `native-tls` feature must be enabled");

mod client;
mod error;
pub mod model;
pub mod service;

#[doc(hidden)]
pub mod prelude;

/// Tracing target for HTTP client operations.
///
/// All client spans and events are emitted under this target when
/// the `tracing` feature is enabled.
#[cfg(feature = "tracing")]
pub(crate) const TRACING_TARGET_CLIENT: &str = "nvisy_sdk::client";

// Re-export client types
pub use client::{
    DEFAULT_BASE_URL, DEFAULT_MAX_RETRIES, DEFAULT_TIMEOUT, DEFAULT_USER_AGENT, Nvisy, NvisyBuilder,
};
// Re-export error types
pub use error::{Error, Result};
