//! Health and monitoring models.

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Service operational status.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    /// Service is operating normally.
    Healthy,
    /// Service is operating with some issues but still functional.
    Degraded,
    /// Service is not operational.
    Unhealthy,
}

/// System monitoring status response.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorStatus {
    /// Timestamp when this status was generated.
    pub checked_at: Timestamp,
    /// Overall system health status.
    pub status: ServiceStatus,
    /// Application version.
    pub version: String,
}

/// Request payload for monitoring status endpoint.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckHealth {
    /// Timeout in milliseconds for health checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Whether to return cached results if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cache: Option<bool>,
}

impl CheckHealth {
    /// Creates a new health check request with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the timeout in milliseconds.
    pub fn timeout(mut self, timeout: i32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets whether to use cached results.
    pub fn use_cache(mut self, use_cache: bool) -> Self {
        self.use_cache = Some(use_cache);
        self
    }
}
