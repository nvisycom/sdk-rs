//! Service for managing runtime runs.

use reqwest::Method;
use uuid::Uuid;

use crate::NvisyRt;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::error::Result;
use crate::model::{NewRun, Pagination, RunDetail, RunList, RunResult};

/// Operations for managing runtime runs.
pub trait RunService {
    /// Runs the full pipeline on uploaded content.
    fn create_run(&self, request: &NewRun) -> impl Future<Output = Result<RunResult>> + Send;

    /// Lists runs with optional filters and pagination.
    fn list_runs(
        &self,
        query: &RunQuery,
        pagination: &Pagination,
    ) -> impl Future<Output = Result<RunList>> + Send;

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
    /// Filter by run status (e.g. `running`, `succeeded`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by actor identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<Uuid>,
}

impl RunService for NvisyRt {
    async fn create_run(&self, request: &NewRun) -> Result<RunResult> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Creating run");

        let response = self
            .send_json(Method::POST, "/api/v1/runs", request)
            .await?;
        let result: RunResult = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, run_id = %result.run_id, "Run created");

        Ok(result)
    }

    async fn list_runs(
        &self,
        query: &RunQuery,
        pagination: &Pagination,
    ) -> Result<RunList> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Listing runs");

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

    async fn get_run(&self, id: Uuid) -> Result<RunDetail> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "Getting run");

        let response = self
            .send(Method::GET, &format!("/api/v1/runs/{id}"))
            .await?;
        Ok(response.json().await?)
    }

    async fn cancel_run(&self, id: Uuid) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "Cancelling run");

        self.send(Method::POST, &format!("/api/v1/runs/{id}/cancel"))
            .await?;
        Ok(())
    }

    async fn delete_run(&self, id: Uuid) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, %id, "Deleting run");

        self.send(Method::DELETE, &format!("/api/v1/runs/{id}"))
            .await?;
        Ok(())
    }

    async fn delete_runs(&self) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Deleting all runs");

        self.send(Method::DELETE, "/api/v1/runs").await?;
        Ok(())
    }
}
