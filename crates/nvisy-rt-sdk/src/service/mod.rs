//! Service modules for interacting with Nvisy Runtime API endpoints.
//!
//! Services are implemented as traits on [`Runtime`], providing
//! a clean and ergonomic API for interacting with different Nvisy Runtime API endpoints.
//!
//! [`Runtime`]: crate::Runtime

mod contexts;
mod files;
mod infra;
mod runs;
#[cfg(feature = "stream")]
mod stream;

pub use self::contexts::*;
pub use self::files::*;
pub use self::infra::*;
pub use self::runs::*;
#[cfg(feature = "stream")]
pub use self::stream::PageStream;
