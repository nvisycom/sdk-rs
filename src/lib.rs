#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod client;
pub mod error;
pub mod model;
pub mod service;

#[doc(hidden)]
pub mod prelude;

// Re-export main types at crate root for convenience
pub use client::{NvisyClient, NvisyConfig, NvisyConfigBuilder};
pub use error::{Error, Result};
