//! Pagination types for list endpoints.

use serde::{Deserialize, Serialize};

/// Paginated response wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    /// Total number of items across all pages.
    pub total: u32,
    /// Whether more items exist beyond this page.
    pub has_more: bool,
    /// Items in the current page.
    pub items: Vec<T>,
}

/// Query parameters for paginated list endpoints.
#[derive(Debug, Default, Clone, Serialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// Number of items to skip.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Maximum number of items to return.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
