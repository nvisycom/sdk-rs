//! Data models for health monitoring.

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Represents the operational status of a service.
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

/// Health status of a single service component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentCheck {
    /// Component name (e.g. `"postgres"`, `"nats"`).
    pub name: String,
    /// Status of this component.
    pub status: ServiceStatus,
}

/// Optional request body for `GET /health`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckHealth {
    /// Whether to return a cached result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_cache: Option<bool>,
}

/// Response body for `GET /health`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    /// Overall service status.
    pub status: ServiceStatus,
    /// Per-component health checks.
    pub checks: Vec<ComponentCheck>,
    /// RFC 3339 timestamp of when the check was performed.
    pub timestamp: Timestamp,
}
