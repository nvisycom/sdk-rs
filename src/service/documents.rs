//! Document service trait.

use std::path::Path;

use async_trait::async_trait;

use crate::error::Result;
use crate::model::{
    CreateDocumentRequest, Document, DocumentVersion, Id, PaginatedResponse, Pagination,
    UpdateDocumentRequest,
};

/// Service for document operations.
#[async_trait]
pub trait DocumentService {
    /// Get a document by ID.
    async fn get(&self, id: &Id) -> Result<Document>;

    /// List documents with optional pagination.
    async fn list(&self, pagination: Option<Pagination>) -> Result<PaginatedResponse<Document>>;

    /// List documents in a workspace.
    async fn list_in_workspace(
        &self,
        workspace_id: &Id,
        pagination: Option<Pagination>,
    ) -> Result<PaginatedResponse<Document>>;

    /// Create document metadata (before upload).
    async fn create(&self, request: CreateDocumentRequest) -> Result<Document>;

    /// Update document metadata.
    async fn update(&self, id: &Id, request: UpdateDocumentRequest) -> Result<Document>;

    /// Delete a document by ID.
    async fn delete(&self, id: &Id) -> Result<()>;

    /// Upload document content from a file path.
    async fn upload(&self, id: &Id, path: &Path) -> Result<Document>;

    /// Upload document content from bytes.
    async fn upload_bytes(&self, id: &Id, content: Vec<u8>) -> Result<Document>;

    /// Download document content to a file path.
    async fn download(&self, id: &Id, path: &Path) -> Result<()>;

    /// Download document content as bytes.
    async fn download_bytes(&self, id: &Id) -> Result<Vec<u8>>;

    /// Get document download URL (for direct browser download).
    async fn download_url(&self, id: &Id) -> Result<String>;

    /// List document versions.
    async fn list_versions(
        &self,
        id: &Id,
        pagination: Option<Pagination>,
    ) -> Result<PaginatedResponse<DocumentVersion>>;

    /// Restore a specific version.
    async fn restore_version(&self, id: &Id, version: u32) -> Result<Document>;
}
