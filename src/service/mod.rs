//! Service modules for interacting with Nvisy API endpoints.
//!
//! Services are implemented as traits on [`NvisyClient`](crate::NvisyClient), providing
//! a clean and ergonomic API for interacting with different Nvisy API endpoints.
//!
//! Import the service traits you need and call methods directly on the client:
//!
//! - [`FilesService`] - File upload, download, and management
//! - [`WorkspacesService`] - Workspace CRUD and notifications
//! - [`IntegrationsService`] - Third-party integrations
//! - [`WebhooksService`] - Webhook management
//! - [`HealthService`] - System health checks

mod files;
mod health;
mod integrations;
mod webhooks;
mod workspaces;

pub use files::{FilesService, ListFilesOptions};
pub use health::HealthService;
pub use integrations::{IntegrationsService, ListIntegrationsOptions};
pub use webhooks::{ListWebhooksOptions, WebhooksService};
pub use workspaces::{ListWorkspacesOptions, WorkspacesService};
