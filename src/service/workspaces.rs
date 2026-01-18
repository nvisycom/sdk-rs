//! Workspace service trait and implementation.

use async_trait::async_trait;

use crate::client::NvisyClient;
use crate::error::Result;
use crate::model::{
    CreateWorkspaceRequest, Id, PaginatedResponse, Pagination, UpdateWorkspaceRequest, Workspace,
};

/// Service for workspace operations.
#[async_trait]
pub trait WorkspaceService {
    /// Get a workspace by ID.
    async fn get(&self, id: &Id) -> Result<Workspace>;

    /// List workspaces with optional pagination.
    async fn list(&self, pagination: Option<Pagination>) -> Result<PaginatedResponse<Workspace>>;

    /// Create a new workspace.
    async fn create(&self, request: CreateWorkspaceRequest) -> Result<Workspace>;

    /// Update an existing workspace.
    async fn update(&self, id: &Id, request: UpdateWorkspaceRequest) -> Result<Workspace>;

    /// Delete a workspace by ID.
    async fn delete(&self, id: &Id) -> Result<()>;
}

#[async_trait]
impl WorkspaceService for NvisyClient {
    async fn get(&self, id: &Id) -> Result<Workspace> {
        self.send(self.get(&format!("/workspaces/{}", id))).await
    }

    async fn list(&self, pagination: Option<Pagination>) -> Result<PaginatedResponse<Workspace>> {
        let mut request = self.get("/workspaces");
        if let Some(ref p) = pagination {
            request = request.query(p);
        }
        self.send(request).await
    }

    async fn create(&self, request: CreateWorkspaceRequest) -> Result<Workspace> {
        self.send_json(self.post("/workspaces"), &request).await
    }

    async fn update(&self, id: &Id, request: UpdateWorkspaceRequest) -> Result<Workspace> {
        self.send_json(self.put(&format!("/workspaces/{}", id)), &request)
            .await
    }

    async fn delete(&self, id: &Id) -> Result<()> {
        self.send_delete(self.delete_req(&format!("/workspaces/{}", id)))
            .await
    }
}
