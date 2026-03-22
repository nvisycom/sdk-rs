//! Service for managing runtime files.

use reqwest::Method;
use uuid::Uuid;

use crate::NvisyRt;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::error::Result;
use crate::model::{File, FileId, NewFile, Page, Pagination};
#[cfg(feature = "stream")]
use crate::service::PageStream;

/// Operations for managing runtime files.
pub trait FileService {
    /// Uploads a new file.
    fn upload_file(&self, request: &NewFile) -> impl Future<Output = Result<FileId>> + Send;

    /// Downloads a file by ID.
    fn download_file(&self, id: Uuid) -> impl Future<Output = Result<File>> + Send;

    /// Lists files with pagination.
    fn list_files(&self, pagination: &Pagination) -> impl Future<Output = Result<Page<Uuid>>> + Send;

    /// Deletes a file by ID.
    fn delete_file(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    /// Deletes all files.
    fn delete_files(&self) -> impl Future<Output = Result<()>> + Send;

    /// Returns a stream that yields file IDs across all pages.
    #[cfg(feature = "stream")]
    fn list_files_stream(&self, page_size: Option<u32>) -> PageStream<Uuid>;
}

impl FileService for NvisyRt {
    async fn upload_file(&self, request: &NewFile) -> Result<FileId> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            filename = ?request.filename,
            content_type = ?request.content_type,
            "Uploading file"
        );

        let response = self
            .send_json(Method::POST, "/api/v1/files", request)
            .await?;
        let created: FileId = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, id = %created.id, "File uploaded");

        Ok(created)
    }

    async fn download_file(&self, id: Uuid) -> Result<File> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "Downloading file");

        let response = self
            .send(Method::GET, &format!("/api/v1/files/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn list_files(&self, pagination: &Pagination) -> Result<Page<Uuid>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Listing files");

        let url = self.resolve_url("/api/v1/files");
        let response = self
            .request(Method::GET, url)
            .query(pagination)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        Ok(response.json().await?)
    }

    async fn delete_file(&self, id: Uuid) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "Deleting file");

        self.send(Method::DELETE, &format!("/api/v1/files/{id}"))
            .await?;
        Ok(())
    }

    async fn delete_files(&self) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Deleting all files");

        self.send(Method::DELETE, "/api/v1/files").await?;
        Ok(())
    }

    #[cfg(feature = "stream")]
    fn list_files_stream(&self, page_size: Option<u32>) -> PageStream<Uuid> {
        let client = self.clone();
        PageStream::new(
            Box::new(move |pagination| {
                let client = client.clone();
                Box::pin(async move { FileService::list_files(&client, &pagination).await })
            }),
            page_size.unwrap_or(100),
        )
    }
}
