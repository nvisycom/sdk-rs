//! Data models for runtime contexts.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::error::Result;

/// Request payload for `POST /api/v1/contexts`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContext {
    /// Actor identity that owns the context.
    pub actor_id: Uuid,
    /// The context to store.
    pub context: Value,
}

impl NewContext {
    /// Creates a new context request.
    pub fn new(actor_id: Uuid, context: Value) -> Self {
        Self { actor_id, context }
    }

    /// Creates a new context request by deserializing JSON bytes.
    pub fn from_bytes(actor_id: Uuid, bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            actor_id,
            context: serde_json::from_slice(bytes)?,
        })
    }
}

/// Response body for `POST /api/v1/contexts`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextId {
    /// Identifier assigned to the uploaded context.
    pub id: Uuid,
}

/// Response body for `GET /api/v1/contexts/{id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    /// Identifier of the context.
    pub id: Uuid,
    /// The stored context.
    pub context: Value,
}

/// Response body for `GET /api/v1/contexts`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextList {
    /// List of context identifiers.
    pub contexts: Vec<Uuid>,
}
