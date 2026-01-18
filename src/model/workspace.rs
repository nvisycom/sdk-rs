//! Workspace-related models.

use serde::{Deserialize, Serialize};

use super::{Id, Timestamp};

/// A workspace containing documents and collaborators.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workspace {
    /// Unique identifier.
    pub id: Id,
    /// Workspace name.
    pub name: String,
    /// Workspace description.
    pub description: Option<String>,
    /// ID of the workspace owner.
    pub owner_id: Id,
    /// When the workspace was created.
    pub created_at: Timestamp,
    /// When the workspace was last updated.
    pub updated_at: Timestamp,
}

/// Request to create a new workspace.
#[derive(Debug, Clone, Serialize)]
pub struct CreateWorkspaceRequest {
    /// Workspace name.
    pub name: String,
    /// Workspace description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request to update an existing workspace.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateWorkspaceRequest {
    /// New workspace name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New workspace description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
