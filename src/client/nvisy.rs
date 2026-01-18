//! Nvisy API client implementation.

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;

use super::NvisyConfig;
use crate::error::Result;
use crate::model::{
    CreateDocumentRequest, CreateWorkspaceRequest, Document, DocumentVersion, Id,
    PaginatedResponse, Pagination, UpdateDocumentRequest, UpdateWorkspaceRequest, Workspace,
};
use crate::service::{DocumentService, WorkspaceService};

/// The main Nvisy API client.
#[derive(Debug, Clone)]
pub struct NvisyClient {
    config: Arc<NvisyConfig>,
    http: Client,
}

impl NvisyClient {
    /// Create a new client with the given configuration.
    pub fn new(config: NvisyConfig) -> Result<Self> {
        let http = config.client().unwrap_or_else(|| {
            Client::builder()
                .timeout(config.timeout())
                .build()
                .expect("Failed to build HTTP client")
        });

        Ok(Self {
            config: Arc::new(config),
            http,
        })
    }

    /// Create a new client with just an API key using default settings.
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self> {
        let config = NvisyConfig::builder().with_api_key(api_key).build()?;
        Self::new(config)
    }

    /// Get a reference to the client configuration.
    pub fn config(&self) -> &NvisyConfig {
        &self.config
    }

    /// Build a GET request.
    fn get(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::GET, path)
    }

    /// Build a POST request.
    fn post(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::POST, path)
    }

    /// Build a PUT request.
    fn put(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::PUT, path)
    }

    /// Build a DELETE request.
    fn delete_req(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::DELETE, path)
    }

    /// Build a request with authentication headers.
    fn request(&self, method: reqwest::Method, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.config.base_url(), path);
        self.http
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.config.api_key()))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
    }

    /// Send a request and parse the response.
    async fn send<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Send a request and parse as a paginated response.
    async fn send_paginated<T: DeserializeOwned>(
        &self,
        mut request: RequestBuilder,
        pagination: Option<Pagination>,
    ) -> Result<PaginatedResponse<T>> {
        if let Some(ref p) = pagination {
            request = request.query(p);
        }
        self.send(request).await
    }

    /// Handle API response.
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let body = response.error_for_status()?.json().await?;
        Ok(body)
    }

    /// Send a request with a JSON body.
    async fn send_json<T, B>(&self, request: RequestBuilder, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        self.send(request.json(body)).await
    }

    /// Send a DELETE request (no response body expected).
    async fn send_delete(&self, request: RequestBuilder) -> Result<()> {
        request.send().await?.error_for_status()?;
        Ok(())
    }

    /// Send request expecting raw bytes response.
    async fn send_bytes(&self, request: RequestBuilder) -> Result<Vec<u8>> {
        let response = request.send().await?.error_for_status()?;
        Ok(response.bytes().await?.to_vec())
    }

    /// Send request expecting a string response.
    async fn send_text(&self, request: RequestBuilder) -> Result<String> {
        let response = request.send().await?.error_for_status()?;
        Ok(response.text().await?)
    }
}

// Workspace service implementation
#[async_trait]
impl WorkspaceService for NvisyClient {
    async fn get(&self, id: &Id) -> Result<Workspace> {
        self.send(self.get(&format!("/workspaces/{}", id))).await
    }

    async fn list(&self, pagination: Option<Pagination>) -> Result<PaginatedResponse<Workspace>> {
        self.send_paginated(self.get("/workspaces"), pagination)
            .await
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

// Document service implementation
#[async_trait]
impl DocumentService for NvisyClient {
    async fn get(&self, id: &Id) -> Result<Document> {
        self.send(self.get(&format!("/documents/{}", id))).await
    }

    async fn list(&self, pagination: Option<Pagination>) -> Result<PaginatedResponse<Document>> {
        self.send_paginated(self.get("/documents"), pagination)
            .await
    }

    async fn list_in_workspace(
        &self,
        workspace_id: &Id,
        pagination: Option<Pagination>,
    ) -> Result<PaginatedResponse<Document>> {
        self.send_paginated(
            self.get(&format!("/workspaces/{}/documents", workspace_id)),
            pagination,
        )
        .await
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
            .http
            .put(format!(
                "{}/documents/{}/content",
                self.config.base_url(),
                id
            ))
            .header("Authorization", format!("Bearer {}", self.config.api_key()))
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
        self.send_paginated(self.get(&format!("/documents/{}/versions", id)), pagination)
            .await
    }

    async fn restore_version(&self, id: &Id, version: u32) -> Result<Document> {
        self.send(self.post(&format!("/documents/{}/versions/{}/restore", id, version)))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = NvisyClient::with_api_key("test-key");
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_config_access() {
        let client = NvisyClient::with_api_key("test-key").unwrap();
        assert_eq!(client.config().api_key(), "test-key");
    }
}
