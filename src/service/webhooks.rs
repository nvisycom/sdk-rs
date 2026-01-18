//! Webhooks API service.
//!
//! This module provides methods for managing workspace webhooks.

use std::future::Future;

use reqwest::Method;
use uuid::Uuid;

use crate::client::NvisyClient;
use crate::error::Result;
use crate::model::{
    CreateWebhook, TestWebhook, UpdateWebhook, Webhook, WebhookResult, WebhooksPage,
};

/// Trait for Webhooks API operations.
pub trait WebhooksService {
    /// Lists webhooks in a workspace with optional pagination.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    /// * `options` - Optional listing options (pagination)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::WebhooksService;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let page = client.list_webhooks(workspace_id, None).await?;
    /// for webhook in page.items {
    ///     println!("Webhook: {} -> {}", webhook.display_name, webhook.url);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn list_webhooks(
        &self,
        workspace_id: Uuid,
        options: Option<ListWebhooksOptions>,
    ) -> impl Future<Output = Result<WebhooksPage>>;

    /// Gets a webhook by ID.
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - The webhook identifier
    fn get_webhook(&self, webhook_id: Uuid) -> impl Future<Output = Result<Webhook>>;

    /// Creates a new webhook in a workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    /// * `request` - The webhook creation request
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::WebhooksService;
    /// use nvisy_sdk::model::{CreateWebhook, WebhookEvent};
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let request = CreateWebhook::new(
    ///     "My Webhook",
    ///     "Notifies on document changes",
    ///     "https://example.com/webhook",
    ///     vec![WebhookEvent::DocumentCreated, WebhookEvent::DocumentUpdated],
    /// );
    /// let webhook = client.create_webhook(workspace_id, request).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn create_webhook(
        &self,
        workspace_id: Uuid,
        request: CreateWebhook,
    ) -> impl Future<Output = Result<Webhook>>;

    /// Updates a webhook.
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - The webhook identifier
    /// * `update` - The update request
    fn update_webhook(
        &self,
        webhook_id: Uuid,
        update: UpdateWebhook,
    ) -> impl Future<Output = Result<Webhook>>;

    /// Deletes a webhook.
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - The webhook identifier
    fn delete_webhook(&self, webhook_id: Uuid) -> impl Future<Output = Result<()>>;

    /// Tests a webhook by sending a test payload.
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - The webhook identifier
    /// * `request` - Optional test request with custom payload
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::WebhooksService;
    /// use nvisy_sdk::model::TestWebhook;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let result = client.test_webhook(webhook_id, None).await?;
    /// println!("Test result: {} ({}ms)", result.status_code, result.response_time_ms);
    /// # Ok(())
    /// # }
    /// ```
    fn test_webhook(
        &self,
        webhook_id: Uuid,
        request: Option<TestWebhook>,
    ) -> impl Future<Output = Result<WebhookResult>>;
}

/// Options for listing webhooks.
#[derive(Clone, Debug, Default)]
pub struct ListWebhooksOptions {
    /// Pagination cursor.
    pub after: Option<String>,
    /// Maximum number of results.
    pub limit: Option<i32>,
}

impl ListWebhooksOptions {
    /// Creates a new options builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the pagination cursor.
    pub fn after(mut self, cursor: impl Into<String>) -> Self {
        self.after = Some(cursor.into());
        self
    }

    /// Sets the maximum number of results.
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl WebhooksService for NvisyClient {
    async fn list_webhooks(
        &self,
        workspace_id: Uuid,
        options: Option<ListWebhooksOptions>,
    ) -> Result<WebhooksPage> {
        let path = format!("/workspaces/{}/webhooks/", workspace_id);
        let opts = options.unwrap_or_default();

        let mut req = self.request_builder(Method::GET, &path)?;

        if let Some(after) = &opts.after {
            req = req.query(&[("after", after)]);
        }
        if let Some(limit) = opts.limit {
            req = req.query(&[("limit", limit)]);
        }

        let response = req.send().await?;
        let response = response.error_for_status()?;
        let page: WebhooksPage = response.json().await?;
        Ok(page)
    }

    async fn get_webhook(&self, webhook_id: Uuid) -> Result<Webhook> {
        let path = format!("/webhooks/{}/", webhook_id);
        let response = self.send(Method::GET, &path).await?;
        let response = response.error_for_status()?;
        let webhook: Webhook = response.json().await?;
        Ok(webhook)
    }

    async fn create_webhook(&self, workspace_id: Uuid, request: CreateWebhook) -> Result<Webhook> {
        let path = format!("/workspaces/{}/webhooks/", workspace_id);
        let response = self.send_json(Method::POST, &path, &request).await?;
        let response = response.error_for_status()?;
        let webhook: Webhook = response.json().await?;
        Ok(webhook)
    }

    async fn update_webhook(&self, webhook_id: Uuid, update: UpdateWebhook) -> Result<Webhook> {
        let path = format!("/webhooks/{}/", webhook_id);
        let response = self.send_json(Method::PATCH, &path, &update).await?;
        let response = response.error_for_status()?;
        let webhook: Webhook = response.json().await?;
        Ok(webhook)
    }

    async fn delete_webhook(&self, webhook_id: Uuid) -> Result<()> {
        let path = format!("/webhooks/{}/", webhook_id);
        let response = self.send(Method::DELETE, &path).await?;
        response.error_for_status()?;
        Ok(())
    }

    async fn test_webhook(
        &self,
        webhook_id: Uuid,
        request: Option<TestWebhook>,
    ) -> Result<WebhookResult> {
        let path = format!("/webhooks/{}/test", webhook_id);
        let response = match request {
            Some(req) => self.send_json(Method::POST, &path, &req).await?,
            None => self.send(Method::POST, &path).await?,
        };
        let response = response.error_for_status()?;
        let result: WebhookResult = response.json().await?;
        Ok(result)
    }
}
