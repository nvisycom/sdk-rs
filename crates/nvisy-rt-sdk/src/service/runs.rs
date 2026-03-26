//! Service for managing runtime runs.

use reqwest::Method;
use uuid::Uuid;

use crate::Runtime;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::error::Result;
use crate::model::{NewRun, Page, Pagination, RunDetail, RunResult, RunStatus, RunSummary};
#[cfg(feature = "stream")]
use crate::service::PageStream;

/// Operations for managing runtime runs.
pub trait RunService {
    /// Runs the full pipeline on uploaded content.
    fn create_run(&self, request: &NewRun) -> impl Future<Output = Result<RunResult>> + Send;

    /// Lists runs with optional filters and pagination.
    fn list_runs(
        &self,
        query: &RunQuery,
        pagination: &Pagination,
    ) -> impl Future<Output = Result<Page<RunSummary>>> + Send;

    /// Returns a stream that yields run summaries across all pages.
    #[cfg(feature = "stream")]
    fn list_runs_stream(&self, query: &RunQuery, page_size: Option<u32>) -> PageStream<RunSummary>;

    /// Retrieves a full run snapshot by ID.
    fn get_run(&self, id: Uuid) -> impl Future<Output = Result<RunDetail>> + Send;

    /// Cancels an in-progress run.
    fn cancel_run(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    /// Deletes a single finished run.
    fn delete_run(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    /// Deletes all finished runs.
    fn delete_runs(&self) -> impl Future<Output = Result<()>> + Send;
}

/// Query parameters for listing runs.
#[derive(Debug, Default, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunQuery {
    /// Filter by run status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RunStatus>,
}

impl RunService for Runtime {
    async fn create_run(&self, request: &NewRun) -> Result<RunResult> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "creating run");

        let response = self
            .send_json(Method::POST, "/api/v1/runs", request)
            .await?;
        Ok(response.json().await?)
    }

    async fn list_runs(
        &self,
        query: &RunQuery,
        pagination: &Pagination,
    ) -> Result<Page<RunSummary>> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "listing runs");

        let url = self.resolve_url("/api/v1/runs");
        let response = self
            .request(Method::GET, url)
            .query(query)
            .query(pagination)
            .send()
            .await?;
        let response = self.check_response(response).await?;
        Ok(response.json().await?)
    }

    #[cfg(feature = "stream")]
    fn list_runs_stream(&self, query: &RunQuery, page_size: Option<u32>) -> PageStream<RunSummary> {
        let client = self.clone();
        let query = query.clone();
        PageStream::new(
            Box::new(move |pagination| {
                let client = client.clone();
                let query = query.clone();
                Box::pin(async move { RunService::list_runs(&client, &query, &pagination).await })
            }),
            page_size.unwrap_or(100),
        )
    }

    async fn get_run(&self, id: Uuid) -> Result<RunDetail> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "getting run");

        let response = self
            .send(Method::GET, &format!("/api/v1/runs/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn cancel_run(&self, id: Uuid) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "cancelling run");

        self.send(Method::POST, &format!("/api/v1/runs/{id}/cancel"))
            .await?;
        Ok(())
    }

    async fn delete_run(&self, id: Uuid) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "deleting run");

        self.send(Method::DELETE, &format!("/api/v1/runs/{id}"))
            .await?;
        Ok(())
    }

    async fn delete_runs(&self) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "deleting all runs");

        self.send(Method::DELETE, "/api/v1/runs").await?;
        Ok(())
    }
}
