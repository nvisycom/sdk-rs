//! Service for retrieving the OpenAPI specification.

use reqwest::Method;
use serde_json::Value;

use crate::NvisyRt;
#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::error::Result;

/// Operations for retrieving the runtime OpenAPI specification.
pub trait SpecService {
    /// Retrieves the OpenAPI specification as JSON.
    fn openapi_spec(&self) -> impl Future<Output = Result<Value>> + Send;
}

impl SpecService for NvisyRt {
    async fn openapi_spec(&self) -> Result<Value> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Getting OpenAPI spec");

        let response = self.send(Method::GET, "/api/v1/openapi.json").await?;
        Ok(response.json().await?)
    }
}
