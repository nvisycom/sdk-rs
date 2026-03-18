//! Service for health and analytics checks.

use reqwest::Method;

use crate::NvisyRt;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::error::Result;
use crate::model::{Analytics, Health};

/// Operations for health and analytics checks.
pub trait CheckService {
    /// Checks the health of the runtime.
    fn health(&self) -> impl Future<Output = Result<Health>> + Send;

    /// Retrieves analytics data.
    fn analytics(&self) -> impl Future<Output = Result<Analytics>> + Send;
}

impl CheckService for NvisyRt {
    async fn health(&self) -> Result<Health> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Health check");

        let response = self.send(Method::GET, "/health").await?;
        Ok(response.json().await?)
    }

    async fn analytics(&self) -> Result<Analytics> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Getting analytics");

        let response = self.send(Method::GET, "/api/v1/analytics").await?;
        Ok(response.json().await?)
    }
}
