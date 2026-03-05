//! Service for managing runtime files.

use reqwest::Method;
use uuid::Uuid;

use crate::NvisyRt;
use crate::error::Result;
use crate::model::{CreatedFile, File, FileList, NewFile};

/// Operations for managing runtime files.
pub trait FileService {
    /// Creates (uploads) a new file.
    fn create_file(&self, request: &NewFile) -> impl Future<Output = Result<CreatedFile>> + Send;

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
    async fn create_file(&self, request: &NewFile) -> Result<CreatedFile> {
        let response = self
            .send_json(Method::POST, "/api/v1/files", request)
            .await?;
        Ok(response.json().await?)
    }

    async fn get_file(&self, id: Uuid) -> Result<File> {
        let response = self
            .send(Method::GET, &format!("/api/v1/files/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn list_files(&self) -> Result<FileList> {
        let response = self.send(Method::GET, "/api/v1/files").await?;
        Ok(response.json().await?)
    }

    async fn delete_file(&self, id: Uuid) -> Result<()> {
        self.send(Method::DELETE, &format!("/api/v1/files/{id}"))
            .await?;
        Ok(())
    }

    async fn delete_files(&self) -> Result<()> {
        self.send(Method::DELETE, "/api/v1/files").await?;
        Ok(())
    }
}
