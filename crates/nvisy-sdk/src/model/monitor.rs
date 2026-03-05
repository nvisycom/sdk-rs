//! Data models for health monitoring.

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Optional request parameters for `GET /health`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckHealth {
    /// Timeout in milliseconds for the health check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// Whether to return a cached result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cache: Option<bool>,
}

impl CheckHealth {
    /// Sets the timeout in milliseconds.
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets whether to use a cached result.
    pub fn with_cache(mut self, use_cache: bool) -> Self {
        self.use_cache = Some(use_cache);
        self
    }
}

/// Response body for `GET /health`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorStatus {
    /// Timestamp when this status was generated.
    pub checked_at: Timestamp,
    /// Overall system health status.
    pub status: ServiceStatus,
    /// Application version.
    pub version: String,
}

/// Overall health status of the service.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStatus {
    /// Service is operating normally.
    #[default]
    Healthy,
    /// Service is operating with some issues but still functional.
    Degraded,
    /// Service is not operational.
    Unhealthy,
}
