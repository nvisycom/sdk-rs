//! Infrastructure response types: health and analytics.

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Represents the operational status of a service.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
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
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ComponentCheck {
    /// Component name (e.g. `"filesystem"`, `"registry"`).
    pub name: String,
    /// Status of this component.
    pub status: ServiceStatus,
}

/// Response body for `GET /health`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Health {
    /// Overall service status.
    pub status: ServiceStatus,
    /// Per-component health checks.
    pub checks: Vec<ComponentCheck>,
    /// RFC 3339 timestamp of when the check was performed.
    #[cfg_attr(feature = "jsonschema", schemars(with = "String"))]
    pub timestamp: Timestamp,
}

/// Response body for `GET /api/v1/analytics`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Analytics {
    /// RFC 3339 timestamp of when the analytics were collected.
    #[cfg_attr(feature = "jsonschema", schemars(with = "String"))]
    pub timestamp: Timestamp,
    /// Total number of pipeline runs.
    pub total_runs: u64,
    /// Number of runs currently in progress.
    pub active_runs: u64,
    /// Number of runs that completed successfully.
    pub succeeded_runs: u64,
    /// Number of runs that failed (fully or partially).
    pub failed_runs: u64,
    /// Number of runs that were cancelled.
    pub cancelled_runs: u64,
    /// Total number of entities detected across all completed runs.
    pub total_entities_detected: u64,
    /// Total number of redactions applied across all completed runs.
    pub total_redactions_applied: u64,
    /// Number of distinct actors that have triggered runs.
    pub distinct_actors: u64,
}
