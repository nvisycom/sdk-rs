//! Data models for runtime contexts.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Request payload for `POST /api/v1/contexts`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContext {
    /// Actor identity that owns the context.
    pub actor_id: Uuid,
    /// The context to store.
    pub context: Value,
}

/// Response body for `POST /api/v1/contexts`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedContext {
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
