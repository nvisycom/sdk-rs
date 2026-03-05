//! Service for managing runtime contexts.

use reqwest::Method;
use uuid::Uuid;

use crate::NvisyRt;
use crate::error::Result;
use crate::model::{Context, ContextList, CreatedContext, DeletedContext, DeletedContexts, NewContext};

/// Operations for managing runtime contexts.
pub trait ContextService {
    /// Creates a new context.
    fn create_context(
        &self,
        request: &NewContext,
    ) -> impl Future<Output = Result<CreatedContext>> + Send;

    /// Retrieves a context by ID.
    fn get_context(&self, id: Uuid) -> impl Future<Output = Result<Context>> + Send;

    /// Lists all contexts.
    fn list_contexts(&self) -> impl Future<Output = Result<ContextList>> + Send;

    /// Deletes a context by ID.
    fn delete_context(&self, id: Uuid) -> impl Future<Output = Result<DeletedContext>> + Send;

    /// Deletes all contexts.
    fn delete_contexts(&self) -> impl Future<Output = Result<DeletedContexts>> + Send;
}

impl ContextService for NvisyRt {
    async fn create_context(&self, request: &NewContext) -> Result<CreatedContext> {
        let response = self
            .send_json(Method::POST, "/api/v1/contexts", request)
            .await?;
        Ok(response.json().await?)
    }

    async fn get_context(&self, id: Uuid) -> Result<Context> {
        let response = self
            .send(Method::GET, &format!("/api/v1/contexts/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn list_contexts(&self) -> Result<ContextList> {
        let response = self.send(Method::GET, "/api/v1/contexts").await?;
        Ok(response.json().await?)
    }

    async fn delete_context(&self, id: Uuid) -> Result<DeletedContext> {
        let response = self
            .send(Method::DELETE, &format!("/api/v1/contexts/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn delete_contexts(&self) -> Result<DeletedContexts> {
        let response = self.send(Method::DELETE, "/api/v1/contexts").await?;
        Ok(response.json().await?)
    }
}
