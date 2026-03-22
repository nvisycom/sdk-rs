//! Data models for runtime runs.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::Page;

/// Paginated list of run summaries.
pub type RunList = Page<RunSummary>;

/// Lifecycle status of a pipeline run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    /// The run has been created but not yet started.
    Pending,
    /// The run is actively executing nodes.
    Running,
    /// All nodes completed without error.
    Succeeded,
    /// Some nodes succeeded while others failed.
    PartialFailure,
    /// All nodes failed.
    Failed,
    /// The run was cancelled by the caller.
    Cancelled,
}

/// Request payload for `POST /api/v1/runs`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRun {
    /// Policies to apply during processing.
    pub policies: Value,
    /// Execution graph defining the pipeline DAG.
    pub graph: Value,
    /// Per-request configuration overrides (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
}

/// Response body for `POST /api/v1/runs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    /// Unique run identifier.
    pub run_id: Uuid,
    /// Full detection result (entities, sensitivity, risk).
    pub detection: Value,
    /// Policy evaluation breakdown (decisions, reviews, suppressions).
    pub evaluation: Value,
    /// Per-source redaction summaries.
    pub summaries: Vec<Value>,
    /// Per-file processing audit trails.
    pub audits: Vec<Value>,
    /// Redaction mapping artifacts.
    pub redaction_maps: Vec<Value>,
}

/// Response body for `GET /api/v1/runs/{id}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunDetail {
    /// Full run snapshot (flattened).
    #[serde(flatten)]
    pub run: Value,
}

/// Summary of a single run returned in list responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunSummary {
    /// Run identifier.
    pub id: Uuid,
    /// Additional run metadata (flattened).
    #[serde(flatten)]
    pub extra: Value,
}
