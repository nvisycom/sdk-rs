//! Files API service.
//!
//! This module provides methods for managing files in workspaces.

use std::future::Future;

use reqwest::Method;
use reqwest::multipart::{Form, Part};
use uuid::Uuid;

use crate::client::NvisyClient;
use crate::error::Result;
use crate::model::{
    ArchiveFormat, DeleteFiles, DownloadFiles, File, FileFormat, FilesPage, UpdateFile,
};

/// Trait for Files API operations.
pub trait FilesService {
    /// Lists files in a workspace with optional filtering and pagination.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    /// * `options` - Optional listing options (filters, pagination)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::FilesService;
    /// use nvisy_sdk::model::FileFormat;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    ///
    /// // Simple listing
    /// let page = client.list_files("workspace-id", None).await?;
    ///
    /// // With options
    /// let options = nvisy_sdk::service::ListFilesOptions::new()
    ///     .formats(vec![FileFormat::Pdf, FileFormat::Docx])
    ///     .limit(50);
    /// let page = client.list_files("workspace-id", Some(options)).await?;
    ///
    /// for file in page.items {
    ///     println!("File: {} ({})", file.display_name, file.file_id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn list_files(
        &self,
        workspace_id: Uuid,
        options: Option<ListFilesOptions>,
    ) -> impl Future<Output = Result<FilesPage>>;

    /// Gets a file by ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file identifier
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::FilesService;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let file = client.get_file("file-id").await?;
    /// println!("File: {} ({} bytes)", file.display_name, file.file_size);
    /// # Ok(())
    /// # }
    /// ```
    fn get_file(&self, file_id: Uuid) -> impl Future<Output = Result<File>>;

    /// Updates a file's metadata.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file identifier
    /// * `update` - The update request
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::FilesService;
    /// use nvisy_sdk::model::UpdateFile;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let update = UpdateFile {
    ///     display_name: Some("New Name".into()),
    ///     ..Default::default()
    /// };
    /// let file = client.update_file("file-id", update).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn update_file(&self, file_id: Uuid, update: UpdateFile) -> impl Future<Output = Result<File>>;

    /// Deletes a file.
    ///
    /// This performs a soft delete. The file can be recovered within the retention period.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file identifier
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::FilesService;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// client.delete_file("file-id").await?;
    /// # Ok(())
    /// # }
    /// ```
    fn delete_file(&self, file_id: Uuid) -> impl Future<Output = Result<()>>;

    /// Downloads a file's content.
    ///
    /// Returns the raw bytes of the file content.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file identifier
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::FilesService;
    /// use std::fs;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let content = client.download_file("file-id").await?;
    /// fs::write("downloaded-file.pdf", content)?;
    /// # Ok(())
    /// # }
    /// ```
    fn download_file(&self, file_id: Uuid) -> impl Future<Output = Result<Vec<u8>>>;

    /// Uploads a file to a workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    /// * `file_name` - The file name
    /// * `file_data` - The file content as bytes
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::FilesService;
    /// use std::fs;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let content = fs::read("document.pdf")?;
    /// let file = client.upload_file("workspace-id", "document.pdf", content).await?;
    /// println!("Uploaded: {}", file.file_id);
    /// # Ok(())
    /// # }
    /// ```
    fn upload_file(
        &self,
        workspace_id: Uuid,
        file_name: &str,
        file_data: Vec<u8>,
    ) -> impl Future<Output = Result<File>>;

    /// Deletes multiple files in a batch.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    /// * `file_ids` - List of file IDs to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::FilesService;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let file_ids = vec!["file-1".into(), "file-2".into()];
    /// client.delete_files_batch("workspace-id", file_ids).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn delete_files_batch(
        &self,
        workspace_id: Uuid,
        file_ids: Vec<Uuid>,
    ) -> impl Future<Output = Result<()>>;

    /// Downloads multiple files as an archive.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The workspace identifier
    /// * `file_ids` - List of file IDs to download (empty for all files)
    /// * `format` - Archive format (ZIP or TAR.GZ)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::FilesService;
    /// use nvisy_sdk::model::ArchiveFormat;
    /// use std::fs;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    ///
    /// // Download all files as ZIP
    /// let archive = client.download_files_batch("workspace-id", vec![], ArchiveFormat::Zip).await?;
    /// fs::write("files.zip", archive)?;
    ///
    /// // Download specific files as TAR.GZ
    /// let file_ids = vec!["file-1".into(), "file-2".into()];
    /// let archive = client.download_files_batch("workspace-id", file_ids, ArchiveFormat::TarGz).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn download_files_batch(
        &self,
        workspace_id: Uuid,
        file_ids: Vec<Uuid>,
        format: ArchiveFormat,
    ) -> impl Future<Output = Result<Vec<u8>>>;
}

/// Options for listing files.
#[derive(Clone, Debug, Default)]
pub struct ListFilesOptions {
    /// Filter by file formats.
    pub formats: Option<Vec<FileFormat>>,
    /// Search query.
    pub search: Option<String>,
    /// Pagination cursor.
    pub after: Option<String>,
    /// Maximum number of results.
    pub limit: Option<i32>,
}

impl ListFilesOptions {
    /// Creates a new options builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the file formats filter.
    pub fn formats(mut self, formats: Vec<FileFormat>) -> Self {
        self.formats = Some(formats);
        self
    }

    /// Sets the search query.
    pub fn search(mut self, query: impl Into<String>) -> Self {
        self.search = Some(query.into());
        self
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

impl FilesService for NvisyClient {
    async fn list_files(
        &self,
        workspace_id: Uuid,
        options: Option<ListFilesOptions>,
    ) -> Result<FilesPage> {
        let path = format!("/workspaces/{}/files/", workspace_id);
        let opts = options.unwrap_or_default();

        let mut req = self.request_builder(Method::GET, &path)?;

        if let Some(formats) = &opts.formats {
            for format in formats {
                req = req.query(&[("formats", format)]);
            }
        }
        if let Some(search) = &opts.search {
            req = req.query(&[("search", search)]);
        }
        if let Some(after) = &opts.after {
            req = req.query(&[("after", after)]);
        }
        if let Some(limit) = opts.limit {
            req = req.query(&[("limit", limit)]);
        }

        let response = req.send().await?;
        let response = response.error_for_status()?;
        let page: FilesPage = response.json().await?;
        Ok(page)
    }

    async fn get_file(&self, file_id: Uuid) -> Result<File> {
        let path = format!("/files/{}", file_id);
        let response = self.send(Method::GET, &path).await?;
        let response = response.error_for_status()?;
        let file: File = response.json().await?;
        Ok(file)
    }

    async fn update_file(&self, file_id: Uuid, update: UpdateFile) -> Result<File> {
        let path = format!("/files/{}", file_id);
        let response = self.send_json(Method::PATCH, &path, &update).await?;
        let response = response.error_for_status()?;
        let file: File = response.json().await?;
        Ok(file)
    }

    async fn delete_file(&self, file_id: Uuid) -> Result<()> {
        let path = format!("/files/{}", file_id);
        let response = self.send(Method::DELETE, &path).await?;
        response.error_for_status()?;
        Ok(())
    }

    async fn download_file(&self, file_id: Uuid) -> Result<Vec<u8>> {
        let path = format!("/files/{}/content", file_id);
        let response = self.send(Method::GET, &path).await?;
        let response = response.error_for_status()?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    async fn upload_file(
        &self,
        workspace_id: Uuid,
        file_name: &str,
        file_data: Vec<u8>,
    ) -> Result<File> {
        let path = format!("/workspaces/{}/files/", workspace_id);

        let file_part = Part::bytes(file_data).file_name(file_name.to_string());
        let form = Form::new().part("file", file_part);

        let response = self.send_multipart(Method::POST, &path, form).await?;
        let response = response.error_for_status()?;
        let files: Vec<File> = response.json().await?;

        // API returns array of uploaded files, we uploaded one
        files
            .into_iter()
            .next()
            .ok_or_else(|| crate::error::Error::Api("upload returned no files".into()))
    }

    async fn delete_files_batch(&self, workspace_id: Uuid, file_ids: Vec<Uuid>) -> Result<()> {
        let path = format!("/workspaces/{}/files/batch", workspace_id);
        let body = DeleteFiles { file_ids };
        let response = self.send_json(Method::DELETE, &path, &body).await?;
        response.error_for_status()?;
        Ok(())
    }

    async fn download_files_batch(
        &self,
        workspace_id: Uuid,
        file_ids: Vec<Uuid>,
        format: ArchiveFormat,
    ) -> Result<Vec<u8>> {
        let path = format!("/workspaces/{}/files/batch", workspace_id);
        let body = DownloadFiles { file_ids, format };
        let response = self.send_json(Method::GET, &path, &body).await?;
        let response = response.error_for_status()?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
