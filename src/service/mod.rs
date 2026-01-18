//! Service modules for interacting with Nvisy API endpoints.
//!
//! Services are implemented as traits on [`NvisyClient`](crate::NvisyClient), providing
//! a clean and ergonomic API for interacting with different Nvisy API endpoints.
//!
//! # Example
//!
//! ```no_run
//! use nvisy_sdk::{NvisyClient, Result};
//! use nvisy_sdk::service::{FilesService, WorkspacesService};
//! use nvisy_sdk::model::CreateWorkspace;
//!
//! # async fn example() -> Result<()> {
//! let client = NvisyClient::with_api_key("your-api-key")?;
//!
//! // Create a workspace
//! let workspace = client.create_workspace(
//!     CreateWorkspace::new("My Workspace")
//! ).await?;
//!
//! // Upload a file
//! let content = std::fs::read("document.pdf")?;
//! let file = client.upload_file(&workspace.workspace_id, "document.pdf", content).await?;
//!
//! println!("Uploaded file: {}", file.file_id);
//! # Ok(())
//! # }
//! ```

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
