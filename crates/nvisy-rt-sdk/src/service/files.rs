//! Service for managing runtime files.

use reqwest::Method;
use uuid::Uuid;

use crate::NvisyRt;
use crate::error::Result;
use crate::model::{File, FileId, FileList, NewFile};
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;

/// Operations for managing runtime files.
pub trait FileService {
    /// Creates (uploads) a new file.
    fn create_file(&self, request: &NewFile) -> impl Future<Output = Result<FileId>> + Send;

    /// Retrieves a file by ID.
    fn get_file(&self, id: Uuid) -> impl Future<Output = Result<File>> + Send;

    /// Lists all files.
    fn list_files(&self) -> impl Future<Output = Result<FileList>> + Send;

    /// Deletes a file by ID.
    fn delete_file(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    /// Deletes all files.
    fn delete_files(&self) -> impl Future<Output = Result<()>> + Send;
}

impl FileService for NvisyRt {
    async fn create_file(&self, request: &NewFile) -> Result<FileId> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            actor_id = %request.actor_id,
            filename = ?request.filename,
            content_type = ?request.content_type,
            "Creating file"
        );

        let response = self
            .send_json(Method::POST, "/api/v1/files", request)
            .await?;
        let created: FileId = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, id = %created.id, "File created");

        Ok(created)
    }

    async fn get_file(&self, id: Uuid) -> Result<File> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "Getting file");

        let response = self
            .send(Method::GET, &format!("/api/v1/files/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn list_files(&self) -> Result<FileList> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Listing files");

        let response = self.send(Method::GET, "/api/v1/files").await?;
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
}
