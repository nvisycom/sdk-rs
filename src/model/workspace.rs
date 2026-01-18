//! Workspace-related data models.

use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a workspace.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    /// Unique workspace identifier.
    pub workspace_id: Uuid,
    /// Display name of the workspace.
    pub display_name: String,
    /// Description of the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tags associated with the workspace.
    pub tags: Vec<String>,
    /// Whether comments are enabled.
    pub enable_comments: bool,
    /// Whether approval is required for processed files.
    pub require_approval: bool,
    /// Role of the current member in the workspace.
    pub member_role: WorkspaceRole,
    /// Account ID of the creator.
    pub created_by: Uuid,
    /// Creation timestamp.
    pub created_at: Timestamp,
    /// Last update timestamp.
    pub updated_at: Timestamp,
}

/// Role of a member in a workspace.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceRole {
    /// Full access and management rights.
    Owner,
    /// Can manage content and members.
    Admin,
    /// Can view and edit content.
    Editor,
    /// Can only view content.
    Viewer,
}

/// Request body for creating a workspace.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspace {
    /// Display name of the workspace.
    pub display_name: String,
    /// Description of the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tags associated with the workspace.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Whether comments are enabled.
    #[serde(default = "default_true")]
    pub enable_comments: bool,
    /// Whether approval is required for processed files.
    #[serde(default)]
    pub require_approval: bool,
}

fn default_true() -> bool {
    true
}

impl CreateWorkspace {
    /// Creates a new workspace request with just a display name.
    pub fn new(display_name: impl Into<String>) -> Self {
        Self {
            display_name: display_name.into(),
            description: None,
            tags: Vec::new(),
            enable_comments: true,
            require_approval: false,
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the tags.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Sets whether comments are enabled.
    pub fn with_comments(mut self, enabled: bool) -> Self {
        self.enable_comments = enabled;
        self
    }

    /// Sets whether approval is required.
    pub fn with_approval(mut self, required: bool) -> Self {
        self.require_approval = required;
        self
    }
}

/// Request body for updating a workspace.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkspace {
    /// New display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// New description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Whether comments are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_comments: Option<bool>,
    /// Whether approval is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval: Option<bool>,
}

/// Paginated list of workspaces.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspacesPage {
    /// List of workspaces.
    pub items: Vec<Workspace>,
    /// Cursor for the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether there are more results.
    pub has_more: bool,
}

/// Notification settings for a workspace.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    /// Whether email notifications are enabled.
    pub email_enabled: bool,
    /// Whether in-app notifications are enabled.
    pub in_app_enabled: bool,
    /// Events to notify about.
    pub events: Vec<NotificationEvent>,
}

/// Types of notification events.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationEvent {
    /// New comment added.
    CommentAdded,
    /// Reply to a comment.
    CommentReply,
    /// File processing completed.
    FileCompleted,
    /// File processing failed.
    FileFailed,
    /// New member joined.
    MemberJoined,
    /// Member left the workspace.
    MemberLeft,
}

/// Request body for updating notification settings.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationSettings {
    /// Whether email notifications are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_enabled: Option<bool>,
    /// Whether in-app notifications are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_enabled: Option<bool>,
    /// Events to notify about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<NotificationEvent>>,
}
