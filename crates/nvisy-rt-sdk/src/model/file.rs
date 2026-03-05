//! Data models for runtime files.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request payload for `POST /api/v1/files`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFile {
    /// Actor identity that owns the file.
    pub actor_id: Uuid,
    /// Base64-encoded file bytes.
    pub content: String,
    /// Optional original filename.
    #[serde(default)]
    pub filename: Option<String>,
    /// Optional MIME type override (e.g. `text/csv`).
    #[serde(default)]
    pub content_type: Option<String>,
}

/// Response body for `POST /api/v1/files`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedFile {
    /// Identifier assigned to the uploaded file.
    pub id: Uuid,
}

/// Response body for `GET /api/v1/files/{id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// Identifier of the file.
    pub id: Uuid,
    /// Base64-encoded file bytes.
    pub content: String,
}

/// Response body for `GET /api/v1/files`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileList {
    /// List of file identifiers.
    pub files: Vec<Uuid>,
}
