//! Data models for runtime files.

#[cfg(feature = "base64")]
use base64::Engine;
#[cfg(feature = "base64")]
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request payload for `POST /api/v1/files`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFile {
    /// Base64-encoded file bytes.
    pub content: String,
    /// Optional original filename.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Optional MIME type override (e.g. `text/csv`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl NewFile {
    /// Encodes raw bytes as base64 and returns a new `NewFile`.
    #[cfg(feature = "base64")]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileId {
    /// Identifier assigned to the uploaded file.
    pub id: Uuid,
}

/// Response body for `GET /api/v1/files/{id}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// Identifier of the file.
    pub id: Uuid,
    /// Base64-encoded file bytes.
    pub content: String,
    /// MIME type of the file, if provided at upload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Original filename, if provided at upload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[cfg(feature = "base64")]
impl File {
    /// Decodes the base64 content into raw bytes.
    pub fn decode_content(&self) -> crate::Result<Vec<u8>> {
        Ok(STANDARD.decode(&self.content)?)
    }
}
