//! Workspaces API service.
//!
//! This module provides methods for managing workspaces.

use std::future::Future;

use reqwest::Method;
use uuid::Uuid;

use crate::client::NvisyClient;
use crate::error::Result;
use crate::model::{
    CreateWorkspace, NotificationSettings, UpdateNotificationSettings, UpdateWorkspace, Workspace,
    WorkspacesPage,
};

/// Trait for Workspaces API operations.
pub trait WorkspacesService {
    /// Lists all workspaces the authenticated user is a member of.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional listing options (pagination)
    fn list_workspaces(
        &self,
        options: Option<ListWorkspacesOptions>,
    ) -> impl Future<Output = Result<WorkspacesPage>>;

    /// Gets a workspace by ID.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    fn get_workspace(&self, workspace_id: Uuid) -> impl Future<Output = Result<Workspace>>;

    /// Creates a new workspace.
    ///
    /// The creator is automatically added as an owner.
    ///
    /// # Arguments
    ///
    /// * `request` - The workspace creation request
    fn create_workspace(&self, request: CreateWorkspace)
    -> impl Future<Output = Result<Workspace>>;

    /// Updates a workspace.
    ///
    /// Only provided fields are updated.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    /// * `update` - The update request
    fn update_workspace(
        &self,
        workspace_id: Uuid,
        update: UpdateWorkspace,
    ) -> impl Future<Output = Result<Workspace>>;

    /// Deletes a workspace.
    ///
    /// This performs a soft delete. Data is retained for potential recovery.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    fn delete_workspace(&self, workspace_id: Uuid) -> impl Future<Output = Result<()>>;

    /// Gets notification settings for a workspace.
    ///
    /// Returns the notification settings for the authenticated user in the workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    fn get_workspace_notifications(
        &self,
        workspace_id: Uuid,
    ) -> impl Future<Output = Result<NotificationSettings>>;

    /// Updates notification settings for a workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    /// * `update` - The update request
    fn update_workspace_notifications(
        &self,
        workspace_id: Uuid,
        update: UpdateNotificationSettings,
    ) -> impl Future<Output = Result<NotificationSettings>>;
}

/// Options for listing workspaces.
#[derive(Clone, Debug, Default)]
pub struct ListWorkspacesOptions {
    /// Pagination cursor.
    pub after: Option<String>,
    /// Maximum number of results.
    pub limit: Option<i32>,
}

impl ListWorkspacesOptions {
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

impl WorkspacesService for NvisyClient {
    async fn list_workspaces(
        &self,
        options: Option<ListWorkspacesOptions>,
    ) -> Result<WorkspacesPage> {
        let opts = options.unwrap_or_default();
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(after) = &opts.after {
            params.push(("after", after.clone()));
        }
        if let Some(limit) = opts.limit {
            params.push(("limit", limit.to_string()));
        }

        let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let response = self
            .send_with_params(Method::GET, "/workspaces/", &params_ref)
            .await?;
        let response = response.error_for_status()?;
        let page: WorkspacesPage = response.json().await?;
        Ok(page)
    }

    async fn get_workspace(&self, workspace_id: Uuid) -> Result<Workspace> {
        let path = format!("/workspaces/{}/", workspace_id);
        let response = self.send(Method::GET, &path).await?;
        let response = response.error_for_status()?;
        let workspace: Workspace = response.json().await?;
        Ok(workspace)
    }

    async fn create_workspace(&self, request: CreateWorkspace) -> Result<Workspace> {
        let response = self
            .send_json(Method::POST, "/workspaces/", &request)
            .await?;
        let response = response.error_for_status()?;
        let workspace: Workspace = response.json().await?;
        Ok(workspace)
    }

    async fn update_workspace(
        &self,
        workspace_id: Uuid,
        update: UpdateWorkspace,
    ) -> Result<Workspace> {
        let path = format!("/workspaces/{}/", workspace_id);
        let response = self.send_json(Method::PATCH, &path, &update).await?;
        let response = response.error_for_status()?;
        let workspace: Workspace = response.json().await?;
        Ok(workspace)
    }

    async fn delete_workspace(&self, workspace_id: Uuid) -> Result<()> {
        let path = format!("/workspaces/{}/", workspace_id);
        let response = self.send(Method::DELETE, &path).await?;
        response.error_for_status()?;
        Ok(())
    }

    async fn get_workspace_notifications(
        &self,
        workspace_id: Uuid,
    ) -> Result<NotificationSettings> {
        let path = format!("/workspaces/{}/notifications", workspace_id);
        let response = self.send(Method::GET, &path).await?;
        let response = response.error_for_status()?;
        let settings: NotificationSettings = response.json().await?;
        Ok(settings)
    }

    async fn update_workspace_notifications(
        &self,
        workspace_id: Uuid,
        update: UpdateNotificationSettings,
    ) -> Result<NotificationSettings> {
        let path = format!("/workspaces/{}/notifications", workspace_id);
        let response = self.send_json(Method::PATCH, &path, &update).await?;
        let response = response.error_for_status()?;
        let settings: NotificationSettings = response.json().await?;
        Ok(settings)
    }
}
