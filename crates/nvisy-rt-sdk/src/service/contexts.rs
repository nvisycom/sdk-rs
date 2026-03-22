//! Service for managing runtime contexts.

use reqwest::Method;
use uuid::Uuid;

use crate::NvisyRt;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::error::Result;
use crate::model::{Context, ContextId, NewContext, Page, Pagination};
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
    ) -> impl Future<Output = Result<Page<Uuid>>> + Send;

    /// Deletes a context by ID.
    fn delete_context(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    /// Deletes all contexts.
    fn delete_contexts(&self) -> impl Future<Output = Result<()>> + Send;

    /// Returns a stream that yields context IDs across all pages.
    #[cfg(feature = "stream")]
    fn list_contexts_stream(&self, page_size: Option<u32>) -> PageStream<Uuid>;
}

impl ContextService for NvisyRt {
    async fn upload_context(&self, request: &NewContext) -> Result<ContextId> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Uploading context");

        let response = self
            .send_json(Method::POST, "/api/v1/contexts", request)
            .await?;
        let created: ContextId = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, id = %created.id, "Context uploaded");

        Ok(created)
    }

    async fn download_context(&self, id: Uuid) -> Result<Context> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "Downloading context");

        let response = self
            .send(Method::GET, &format!("/api/v1/contexts/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn list_contexts(&self, pagination: &Pagination) -> Result<Page<Uuid>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Listing contexts");

        let url = self.resolve_url("/api/v1/contexts");
        let response = self
            .request(Method::GET, url)
            .query(pagination)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        Ok(response.json().await?)
    }

    async fn delete_context(&self, id: Uuid) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "Deleting context");

        self.send(Method::DELETE, &format!("/api/v1/contexts/{id}"))
            .await?;
        Ok(())
    }

    async fn delete_contexts(&self) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Deleting all contexts");

        self.send(Method::DELETE, "/api/v1/contexts").await?;
        Ok(())
    }

    #[cfg(feature = "stream")]
    fn list_contexts_stream(&self, page_size: Option<u32>) -> PageStream<Uuid> {
        let client = self.clone();
        PageStream::new(
            Box::new(move |pagination| {
                let client = client.clone();
                Box::pin(async move { ContextService::list_contexts(&client, &pagination).await })
            }),
            page_size.unwrap_or(100),
        )
    }
}
