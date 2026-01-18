//! Document service trait and implementation.

use std::path::Path;

use async_trait::async_trait;

use crate::client::NvisyClient;
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

#[async_trait]
impl DocumentService for NvisyClient {
    async fn get(&self, id: &Id) -> Result<Document> {
        self.send(self.get(&format!("/documents/{}", id))).await
    }

    async fn list(&self, pagination: Option<Pagination>) -> Result<PaginatedResponse<Document>> {
        let mut request = self.get("/documents");
        if let Some(ref p) = pagination {
            request = request.query(p);
        }
        self.send(request).await
    }

    async fn list_in_workspace(
        &self,
        workspace_id: &Id,
        pagination: Option<Pagination>,
    ) -> Result<PaginatedResponse<Document>> {
        let mut request = self.get(&format!("/workspaces/{}/documents", workspace_id));
        if let Some(ref p) = pagination {
            request = request.query(p);
        }
        self.send(request).await
    }

    async fn create(&self, request: CreateDocumentRequest) -> Result<Document> {
        self.send_json(self.post("/documents"), &request).await
    }

    async fn update(&self, id: &Id, request: UpdateDocumentRequest) -> Result<Document> {
        self.send_json(self.put(&format!("/documents/{}", id)), &request)
            .await
    }

    async fn delete(&self, id: &Id) -> Result<()> {
        self.send_delete(self.delete_req(&format!("/documents/{}", id)))
            .await
    }

    async fn upload(&self, id: &Id, path: &Path) -> Result<Document> {
        let content = tokio::fs::read(path).await?;
        self.upload_bytes(id, content).await
    }

    async fn upload_bytes(&self, id: &Id, content: Vec<u8>) -> Result<Document> {
        let request = self
            .inner
            .http
            .put(format!(
                "{}/documents/{}/content",
                self.inner.config.base_url(),
                id
            ))
            .header(
                "Authorization",
                format!("Bearer {}", self.inner.config.api_key()),
            )
            .header("Content-Type", "application/octet-stream")
            .body(content);

        self.send(request).await
    }

    async fn download(&self, id: &Id, path: &Path) -> Result<()> {
        let bytes = self.download_bytes(id).await?;
        tokio::fs::write(path, bytes).await?;
        Ok(())
    }

    async fn download_bytes(&self, id: &Id) -> Result<Vec<u8>> {
        self.send_bytes(self.get(&format!("/documents/{}/content", id)))
            .await
    }

    async fn download_url(&self, id: &Id) -> Result<String> {
        self.send_text(self.get(&format!("/documents/{}/url", id)))
            .await
    }

    async fn list_versions(
        &self,
        id: &Id,
        pagination: Option<Pagination>,
    ) -> Result<PaginatedResponse<DocumentVersion>> {
        let mut request = self.get(&format!("/documents/{}/versions", id));
        if let Some(ref p) = pagination {
            request = request.query(p);
        }
        self.send(request).await
    }

    async fn restore_version(&self, id: &Id, version: u32) -> Result<Document> {
        self.send(self.post(&format!("/documents/{}/versions/{}/restore", id, version)))
            .await
    }
}
