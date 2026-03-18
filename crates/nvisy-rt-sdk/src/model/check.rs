//! Data models for health and analytics checks.

use serde::{Deserialize, Serialize};

/// Response body for `GET /health`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    /// Whether the service is healthy.
    pub healthy: bool,
}

/// Response body for `GET /api/v1/analytics`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Analytics {
    // TODO: define analytics fields
}
