//! Service modules for interacting with Nvisy Server API endpoints.
//!
//! Services are implemented as traits on [`Nvisy`], providing
//! a clean and ergonomic API for interacting with different endpoints.
//!
//! [`Nvisy`]: crate::Nvisy

mod monitors;

pub use self::monitors::*;
