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

/// Tracing target for client operations.
#[cfg(feature = "tracing")]
pub(crate) const TRACING_TARGET_CLIENT: &str = "nvisy_rt_sdk::client";

// Re-export client types
pub use client::{DEFAULT_BASE_URL, DEFAULT_TIMEOUT, NvisyRt, NvisyRtBuilder};
// Re-export error types
pub use error::{Error, Result};
