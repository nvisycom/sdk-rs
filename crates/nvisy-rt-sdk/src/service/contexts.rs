//! Service for managing runtime contexts.

use reqwest::Method;
use uuid::Uuid;

use crate::NvisyRt;
use crate::error::Result;
use crate::model::{Context, ContextId, ContextList, NewContext};
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;

/// Operations for managing runtime contexts.
pub trait ContextService {
    /// Creates a new context.
    fn create_context(
        &self,
        request: &NewContext,
    ) -> impl Future<Output = Result<ContextId>> + Send;

    /// Retrieves a context by ID.
    fn get_context(&self, id: Uuid) -> impl Future<Output = Result<Context>> + Send;

    /// Lists all contexts.
    fn list_contexts(&self) -> impl Future<Output = Result<ContextList>> + Send;

    /// Deletes a context by ID.
    fn delete_context(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    /// Deletes all contexts.
    fn delete_contexts(&self) -> impl Future<Output = Result<()>> + Send;
}

impl ContextService for NvisyRt {
    async fn create_context(&self, request: &NewContext) -> Result<ContextId> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, actor_id = %request.actor_id, "Creating context");

        let response = self
            .send_json(Method::POST, "/api/v1/contexts", request)
            .await?;
        let created: ContextId = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, id = %created.id, "Context created");

        Ok(created)
    }

    async fn get_context(&self, id: Uuid) -> Result<Context> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "Getting context");

        let response = self
            .send(Method::GET, &format!("/api/v1/contexts/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn list_contexts(&self) -> Result<ContextList> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Listing contexts");

        let response = self.send(Method::GET, "/api/v1/contexts").await?;
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
}
