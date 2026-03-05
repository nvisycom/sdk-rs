//! Convenient re-exports for common types.
//!
//! This module provides a single import for commonly used types.
//!
//! ```ignore
//! use nvisy_sdk::prelude::*;
//! ```

pub use crate::client::{NvisyClient, NvisyConfig, NvisyConfigBuilder};
pub use crate::error::{Error, Result};
