//! Service for managing runtime runs.

use reqwest::Method;
use uuid::Uuid;

use crate::NvisyRt;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::error::Result;
use crate::model::{NewRun, Run, RunList};

/// Operations for managing runtime runs.
pub trait RunService {
    /// Creates a new run.
    fn create_run(&self, request: &NewRun) -> impl Future<Output = Result<Run>> + Send;

    /// Lists all runs.
    fn list_runs(&self) -> impl Future<Output = Result<RunList>> + Send;

    /// Retrieves a run by ID.
    fn get_run(&self, id: Uuid) -> impl Future<Output = Result<Run>> + Send;

    /// Cancels a run by ID.
    fn cancel_run(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    /// Deletes a run by ID.
    fn delete_run(&self, id: Uuid) -> impl Future<Output = Result<()>> + Send;

    /// Deletes all runs.
    fn delete_runs(&self) -> impl Future<Output = Result<()>> + Send;
}

impl RunService for NvisyRt {
    async fn create_run(&self, request: &NewRun) -> Result<Run> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET_SERVICE,
            actor_id = %request.actor_id,
            "Creating run"
        );

        let response = self
            .send_json(Method::POST, "/api/v1/runs", request)
            .await?;
        let created: Run = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, id = %created.id, "Run created");

        Ok(created)
    }

    async fn list_runs(&self) -> Result<RunList> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Listing runs");

        let response = self.send(Method::GET, "/api/v1/runs").await?;
        Ok(response.json().await?)
    }

    async fn get_run(&self, id: Uuid) -> Result<Run> {
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

        self.send_json(Method::POST, &format!("/api/v1/runs/{id}/cancel"), &())
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
