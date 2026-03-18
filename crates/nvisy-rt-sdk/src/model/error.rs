//! Structured API error returned by the runtime server.

use serde::{Deserialize, Serialize};

/// Structured error returned by the runtime API.
///
/// When the server responds with a non-success status code, the response
/// body contains a JSON object matching this structure.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    /// HTTP status code of the response.
    pub status: u16,
    /// Classification of the error.
    pub kind: ErrorKind,
    /// The resource involved in the error, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Additional context describing what went wrong.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Human-readable error message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Suggested action to resolve the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.kind.as_str())?;
        if let Some(ref msg) = self.message {
            write!(f, ": {msg}")?;
        }
        Ok(())
    }
}

/// Classification of API errors.
///
/// Each variant corresponds to a specific HTTP status code and error scenario.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// 400 Bad Request: missing required path parameter.
    MissingPathParam,
    /// 400 Bad Request: invalid request data.
    BadRequest,
    /// 401 Unauthorized: missing authentication token.
    MissingAuthToken,
    /// 401 Unauthorized: malformed authentication token.
    MalformedAuthToken,
    /// 401 Unauthorized: invalid credentials.
    Unauthorized,
    /// 403 Forbidden: access denied.
    Forbidden,
    /// 404 Not Found: resource not found.
    NotFound,
    /// 409 Conflict: conflicting resource state.
    Conflict,
    /// 429 Too Many Requests: rate limit exceeded.
    TooManyRequests,
    /// 500 Internal Server Error: unexpected server error.
    #[default]
    InternalServerError,
    /// 501 Not Implemented: feature not yet implemented.
    NotImplemented,
}

impl ErrorKind {
    /// Returns the string representation of this error kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingPathParam => "missing_path_param",
            Self::BadRequest => "bad_request",
            Self::MissingAuthToken => "missing_auth_token",
            Self::MalformedAuthToken => "malformed_auth_token",
            Self::Unauthorized => "unauthorized",
            Self::Forbidden => "forbidden",
            Self::NotFound => "not_found",
            Self::Conflict => "conflict",
            Self::TooManyRequests => "too_many_requests",
            Self::InternalServerError => "internal_server_error",
            Self::NotImplemented => "not_implemented",
        }
    }
}
