//! Service for managing runtime contexts.

use reqwest::Method;
use uuid::Uuid;

use crate::Runtime;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::error::Result;
use crate::model::{Context, ContextEntry, ContextId, NewContext, Page, Pagination};
#[cfg(feature = "stream")]
use crate::service::PageStream;

/// Operations for managing runtime contexts.
pub trait ContextService {
    /// Uploads a new context.
    fn upload_context(
        &self,
        request: &NewContext,
    ) -> impl Future<Output = Result<ContextId>> + Send;

    /// Downloads a context by ID.
    fn download_context(&self, id: Uuid) -> impl Future<Output = Result<Context>> + Send;

    /// Lists contexts with pagination.
    fn list_contexts(
        &self,
        pagination: &Pagination,
    ) -> impl Future<Output = Result<Page<ContextEntry>>> + Send;

    /// Returns a stream that yields context entries across all pages.
    #[cfg(feature = "stream")]
    fn list_contexts_stream(&self, page_size: Option<u32>) -> PageStream<ContextEntry>;

    /// Deletes a context by ID.
    fn delete_context(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    /// Deletes all contexts.
    fn delete_contexts(&self) -> impl Future<Output = Result<()>> + Send;
}

impl ContextService for Runtime {
    async fn upload_context(&self, request: &NewContext) -> Result<ContextId> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "uploading context");

        let response = self
            .send_json(Method::POST, "/api/v1/contexts", request)
            .await?;
        Ok(response.json().await?)
    }

    async fn download_context(&self, id: Uuid) -> Result<Context> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "downloading context");

        let response = self
            .send(Method::GET, &format!("/api/v1/contexts/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn list_contexts(&self, pagination: &Pagination) -> Result<Page<ContextEntry>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "listing contexts");

        let url = self.resolve_url("/api/v1/contexts");
        let response = self
            .request(Method::GET, url)
            .query(pagination)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        Ok(response.json().await?)
    }

    #[cfg(feature = "stream")]
    fn list_contexts_stream(&self, page_size: Option<u32>) -> PageStream<ContextEntry> {
        let client = self.clone();
        PageStream::new(
            Box::new(move |pagination| {
                let client = client.clone();
                Box::pin(async move { ContextService::list_contexts(&client, &pagination).await })
            }),
            page_size.unwrap_or(100),
        )
    }

    async fn delete_context(&self, id: Uuid) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "deleting context");

        self.send(Method::DELETE, &format!("/api/v1/contexts/{id}"))
            .await?;
        Ok(())
    }

    async fn delete_contexts(&self) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "deleting all contexts");

        self.send(Method::DELETE, "/api/v1/contexts").await?;
        Ok(())
    }
}
