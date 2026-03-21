//! Data models for runtime runs.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request payload for `POST /api/v1/runs`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRun {}

/// Response body for `POST /api/v1/runs` and `GET /api/v1/runs/{id}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    /// Identifier of the run.
    pub id: Uuid,
}

/// Response body for `GET /api/v1/runs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunList {
    /// List of runs.
    pub runs: Vec<Uuid>,
}
