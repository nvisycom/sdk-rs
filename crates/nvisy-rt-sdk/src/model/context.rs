//! Data models for runtime contexts.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Request payload for `POST /api/v1/contexts`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContext {
    /// The context to store.
    pub context: Value,
}

impl NewContext {
    /// Creates a new context request.
    pub fn new(context: Value) -> Self {
        Self { context }
    }

    /// Creates a new context request by deserializing JSON bytes.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        Ok(Self {
            context: serde_json::from_slice(bytes)?,
        })
    }
}

/// Response body for `POST /api/v1/contexts`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextId {
    /// Identifier assigned to the uploaded context.
    pub id: Uuid,
}

/// Response body for `GET /api/v1/contexts/{id}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    /// Identifier of the context.
    pub id: Uuid,
    /// The stored context.
    pub context: Value,
}
