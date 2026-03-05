//! Service for health monitoring.

use reqwest::Method;

use crate::Nvisy;
use crate::error::Result;
use crate::model::{CheckHealth, MonitorStatus};
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;

/// Operations for health monitoring.
pub trait MonitorService {
    /// Checks the health status of the Nvisy Server.
    fn health_status(
        &self,
        options: Option<CheckHealth>,
    ) -> impl Future<Output = Result<MonitorStatus>> + Send;
}

impl MonitorService for Nvisy {
    async fn health_status(&self, options: Option<CheckHealth>) -> Result<MonitorStatus> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Checking health status");

        let response = match options {
            Some(opts) => {
                self.send_json(Method::GET, "/health", &opts)
                    .await?
            }
            None => self.send(Method::GET, "/health").await?,
        };
        let response = response.error_for_status()?;
        Ok(response.json().await?)
    }
}
