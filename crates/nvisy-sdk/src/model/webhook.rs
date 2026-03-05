//! Webhook models.

use std::collections::HashMap;

use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Defines the types of events that can trigger webhook delivery.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    /// A new document was created.
    DocumentCreated,
    /// A document was updated.
    DocumentUpdated,
    /// A document was deleted.
    DocumentDeleted,
    /// A new file was created.
    FileCreated,
    /// A file was updated.
    FileUpdated,
    /// A file was deleted.
    FileDeleted,
    /// A member was added to the workspace.
    MemberAdded,
    /// A member was deleted from the workspace.
    MemberDeleted,
    /// A member's details were updated.
    MemberUpdated,
    /// An integration was created.
    IntegrationCreated,
    /// An integration was updated.
    IntegrationUpdated,
    /// An integration was deleted.
    IntegrationDeleted,
    /// An integration was synchronized.
    IntegrationSynced,
    /// An integration was desynchronized.
    IntegrationDesynced,
}

/// Defines the operational status of a workspace webhook.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookStatus {
    /// Webhook is active and will receive events.
    Active,
    /// Webhook is temporarily paused.
    Paused,
    /// Webhook is disabled (e.g., too many failures).
    Disabled,
}

/// Origin type of the webhook.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookType {
    /// User-provided webhook.
    Provided,
    /// Integration-created webhook.
    Integration,
}

/// Workspace webhook response.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    /// Unique webhook identifier.
    pub webhook_id: Uuid,
    /// Reference to the workspace this webhook belongs to.
    pub workspace_id: Uuid,
    /// Human-readable name for the webhook.
    pub display_name: String,
    /// Detailed description of the webhook's purpose.
    pub description: String,
    /// The URL to send webhook payloads to.
    pub url: String,
    /// List of event types this webhook receives.
    pub events: Vec<WebhookEvent>,
    /// Custom headers included in webhook requests.
    pub headers: HashMap<String, String>,
    /// Current status of the webhook.
    pub status: WebhookStatus,
    /// Origin type of the webhook.
    pub webhook_type: WebhookType,
    /// Reference to integration (present for integration type webhooks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<Uuid>,
    /// Timestamp of the most recent webhook trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered_at: Option<Timestamp>,
    /// Account that originally created this webhook.
    pub created_by: Uuid,
    /// Timestamp when this webhook was first created.
    pub created_at: Timestamp,
    /// Timestamp when this webhook was last modified.
    pub updated_at: Timestamp,
}

/// Paginated list of webhooks.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhooksPage {
    /// Items in this page.
    pub items: Vec<Webhook>,
    /// Cursor to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Total count of items matching the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// Request payload for creating a new workspace webhook.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhook {
    /// Human-readable name for the webhook (1-100 characters).
    pub display_name: String,
    /// Detailed description of the webhook's purpose (max 500 characters).
    pub description: String,
    /// The URL to send webhook payloads to.
    pub url: String,
    /// List of event types this webhook should receive.
    pub events: Vec<WebhookEvent>,
    /// Optional custom headers to include in webhook requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// Initial status for the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookStatus>,
}

impl CreateWebhook {
    /// Creates a new webhook request.
    pub fn new(
        display_name: impl Into<String>,
        description: impl Into<String>,
        url: impl Into<String>,
        events: Vec<WebhookEvent>,
    ) -> Self {
        Self {
            display_name: display_name.into(),
            description: description.into(),
            url: url.into(),
            events,
            headers: None,
            status: None,
        }
    }

    /// Sets custom headers.
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Sets the initial status.
    pub fn status(mut self, status: WebhookStatus) -> Self {
        self.status = Some(status);
        self
    }
}

/// Request payload for updating an existing workspace webhook.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhook {
    /// Updated human-readable name for the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Updated list of event types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WebhookEvent>>,
    /// Updated custom headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// Updated status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookStatus>,
}

/// Request payload for testing a webhook.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestWebhook {
    /// Custom payload to send in the test.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

impl TestWebhook {
    /// Creates a new test webhook request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a custom payload.
    pub fn payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = Some(payload);
        self
    }
}

/// Result of a webhook delivery attempt.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookResult {
    /// HTTP status code returned by the webhook endpoint.
    pub status_code: i32,
    /// Time taken to receive a response in milliseconds.
    pub response_time_ms: i64,
}
