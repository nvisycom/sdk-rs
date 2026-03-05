//! Data models for runtime files.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};

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

impl NewFile {
    /// Encodes raw bytes as base64 and returns a new `NewFile`.
    pub fn from_bytes(actor_id: Uuid, bytes: &[u8]) -> Self {
        Self {
            actor_id,
            content: STANDARD.encode(bytes),
            filename: None,
            content_type: None,
        }
    }

    /// Sets the original filename.
    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    /// Sets the MIME type (e.g. `text/csv`, `application/pdf`).
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }
}

/// Response body for `POST /api/v1/files`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileId {
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

impl File {
    /// Decodes the base64 content into raw bytes.
    pub fn decode_content(&self) -> Result<Vec<u8>> {
        STANDARD
            .decode(&self.content)
            .map_err(|e| Error::Api(format!("base64 decode error: {e}")))
    }
}

/// Response body for `GET /api/v1/files`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileList {
    /// List of file identifiers.
    pub files: Vec<Uuid>,
}
