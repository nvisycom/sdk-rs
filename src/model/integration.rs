//! Integration models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Defines the functional category of a workspace integration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationType {
    /// Files/documents (Drive, S3, SharePoint, Dropbox).
    Storage,
    /// Email, chat (Gmail, Slack, Teams).
    Communication,
    /// CRM, finance, legal (Salesforce, QuickBooks).
    Business,
    /// Data platforms (Snowflake, Tableau, Looker).
    Analytics,
    /// No-code automation (Zapier, Make).
    Automation,
    /// API/webhook integrations.
    Custom,
    /// Specialized verticals (healthcare, insurance).
    Industry,
}

/// Defines the operational status of a workspace integration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationStatus {
    /// Integration is pending configuration or activation.
    Pending,
    /// Integration is actively running and operational.
    Running,
    /// Integration has been cancelled.
    Cancelled,
}

/// Workspace integration response.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integration {
    /// Unique integration identifier.
    pub integration_id: Uuid,
    /// Reference to the workspace this integration belongs to.
    pub workspace_id: Uuid,
    /// Human-readable name for the integration.
    pub integration_name: String,
    /// Detailed description of the integration's purpose and functionality.
    pub description: String,
    /// Type of third-party service this integration connects to.
    pub integration_type: IntegrationType,
    /// Whether the integration is currently active and enabled.
    pub is_active: bool,
    /// Current sync status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<IntegrationStatus>,
    /// Timestamp of the most recent successful synchronization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_at: Option<DateTime<Utc>>,
    /// Account that originally created this integration.
    pub created_by: Uuid,
    /// Timestamp when this integration was first created.
    pub created_at: DateTime<Utc>,
    /// Timestamp when this integration was last modified.
    pub updated_at: DateTime<Utc>,
}

/// Paginated list of integrations.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationsPage {
    /// Items in this page.
    pub items: Vec<Integration>,
    /// Cursor to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Total count of items matching the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// Request payload for creating a new workspace integration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIntegration {
    /// Human-readable name for the integration (1-100 characters).
    pub integration_name: String,
    /// Detailed description of the integration's purpose (1-500 characters).
    pub description: String,
    /// Type of third-party service this integration connects to.
    pub integration_type: IntegrationType,
    /// Integration credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,
    /// Whether the integration should be active immediately upon creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Additional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl CreateIntegration {
    /// Creates a new integration request.
    pub fn new(
        integration_name: impl Into<String>,
        description: impl Into<String>,
        integration_type: IntegrationType,
    ) -> Self {
        Self {
            integration_name: integration_name.into(),
            description: description.into(),
            integration_type,
            credentials: None,
            is_active: None,
            metadata: None,
        }
    }

    /// Sets the credentials.
    pub fn credentials(mut self, credentials: serde_json::Value) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Sets whether the integration should be active immediately.
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    /// Sets additional metadata.
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Request payload for updating an existing workspace integration.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIntegration {
    /// Updated human-readable name for the integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    /// Updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated integration type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<IntegrationType>,
    /// Updated credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,
    /// Updated active status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Updated metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
