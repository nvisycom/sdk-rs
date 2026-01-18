//! Integrations API service.
//!
//! This module provides methods for managing workspace integrations.

use std::future::Future;

use reqwest::Method;
use uuid::Uuid;

use crate::client::NvisyClient;
use crate::error::Result;
use crate::model::{CreateIntegration, Integration, IntegrationsPage, UpdateIntegration};

/// Trait for Integrations API operations.
pub trait IntegrationsService {
    /// Lists integrations in a workspace with optional pagination.
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
    /// use nvisy_sdk::service::IntegrationsService;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let page = client.list_integrations(workspace_id, None).await?;
    /// for integration in page.items {
    ///     println!("Integration: {} ({:?})", integration.integration_name, integration.integration_type);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn list_integrations(
        &self,
        workspace_id: Uuid,
        options: Option<ListIntegrationsOptions>,
    ) -> impl Future<Output = Result<IntegrationsPage>>;

    /// Gets an integration by ID.
    ///
    /// # Arguments
    ///
    /// * `integration_id` - The integration identifier
    fn get_integration(&self, integration_id: Uuid) -> impl Future<Output = Result<Integration>>;

    /// Creates a new integration in a workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    /// * `request` - The integration creation request
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::IntegrationsService;
    /// use nvisy_sdk::model::{CreateIntegration, IntegrationType};
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let request = CreateIntegration::new(
    ///     "My S3 Integration",
    ///     "Sync documents from S3 bucket",
    ///     IntegrationType::Storage,
    /// );
    /// let integration = client.create_integration(workspace_id, request).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn create_integration(
        &self,
        workspace_id: Uuid,
        request: CreateIntegration,
    ) -> impl Future<Output = Result<Integration>>;

    /// Updates an integration.
    ///
    /// # Arguments
    ///
    /// * `integration_id` - The integration identifier
    /// * `update` - The update request
    fn update_integration(
        &self,
        integration_id: Uuid,
        update: UpdateIntegration,
    ) -> impl Future<Output = Result<Integration>>;

    /// Deletes an integration.
    ///
    /// # Arguments
    ///
    /// * `integration_id` - The integration identifier
    fn delete_integration(&self, integration_id: Uuid) -> impl Future<Output = Result<()>>;

    /// Triggers a sync for an integration.
    ///
    /// # Arguments
    ///
    /// * `integration_id` - The integration identifier
    fn sync_integration(&self, integration_id: Uuid) -> impl Future<Output = Result<Integration>>;
}

/// Options for listing integrations.
#[derive(Clone, Debug, Default)]
pub struct ListIntegrationsOptions {
    /// Pagination cursor.
    pub after: Option<String>,
    /// Maximum number of results.
    pub limit: Option<i32>,
}

impl ListIntegrationsOptions {
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

impl IntegrationsService for NvisyClient {
    async fn list_integrations(
        &self,
        workspace_id: Uuid,
        options: Option<ListIntegrationsOptions>,
    ) -> Result<IntegrationsPage> {
        let path = format!("/workspaces/{}/integrations/", workspace_id);
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
        let page: IntegrationsPage = response.json().await?;
        Ok(page)
    }

    async fn get_integration(&self, integration_id: Uuid) -> Result<Integration> {
        let path = format!("/integrations/{}/", integration_id);
        let response = self.send(Method::GET, &path).await?;
        let response = response.error_for_status()?;
        let integration: Integration = response.json().await?;
        Ok(integration)
    }

    async fn create_integration(
        &self,
        workspace_id: Uuid,
        request: CreateIntegration,
    ) -> Result<Integration> {
        let path = format!("/workspaces/{}/integrations/", workspace_id);
        let response = self.send_json(Method::POST, &path, &request).await?;
        let response = response.error_for_status()?;
        let integration: Integration = response.json().await?;
        Ok(integration)
    }

    async fn update_integration(
        &self,
        integration_id: Uuid,
        update: UpdateIntegration,
    ) -> Result<Integration> {
        let path = format!("/integrations/{}/", integration_id);
        let response = self.send_json(Method::PATCH, &path, &update).await?;
        let response = response.error_for_status()?;
        let integration: Integration = response.json().await?;
        Ok(integration)
    }

    async fn delete_integration(&self, integration_id: Uuid) -> Result<()> {
        let path = format!("/integrations/{}/", integration_id);
        let response = self.send(Method::DELETE, &path).await?;
        response.error_for_status()?;
        Ok(())
    }

    async fn sync_integration(&self, integration_id: Uuid) -> Result<Integration> {
        let path = format!("/integrations/{}/sync", integration_id);
        let response = self.send(Method::POST, &path).await?;
        let response = response.error_for_status()?;
        let integration: Integration = response.json().await?;
        Ok(integration)
    }
}
