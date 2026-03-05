//! Service modules for interacting with Nvisy Server API endpoints.
//!
//! Services are implemented as traits on [`Nvisy`], providing
//! a clean and ergonomic API for interacting with different endpoints.
//!
//! Import the service traits you need and call methods directly on the client:
//!
//! - [`WorkspacesService`]: Workspace CRUD and notifications
//!
//! [`Nvisy`]: crate::Nvisy

mod workspaces;

pub use workspaces::{ListWorkspacesOptions, WorkspacesService};
