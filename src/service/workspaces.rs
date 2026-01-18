//! Workspace service trait.

use async_trait::async_trait;

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
