//! Data models for the Nvisy API.

mod document;
mod workspace;

pub use document::*;
pub use workspace::*;

use serde::{Deserialize, Serialize};

/// Common ID type used across the API.
pub type Id = String;

/// Timestamp type (ISO 8601 format).
pub type Timestamp = String;

/// Pagination parameters for list requests.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Pagination {
    /// Number of items to skip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl Pagination {
    /// Create a new pagination with offset and limit.
    pub fn new(offset: u32, limit: u32) -> Self {
        Self {
            offset: Some(offset),
            limit: Some(limit),
        }
    }

    /// Create pagination for a specific page.
    pub fn page(page: u32, per_page: u32) -> Self {
        Self {
            offset: Some(page.saturating_sub(1) * per_page),
            limit: Some(per_page),
        }
    }
}

/// Paginated response wrapper.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The items in this page.
    pub data: Vec<T>,
    /// Total number of items available.
    pub total: u32,
    /// Current offset.
    pub offset: u32,
    /// Current limit.
    pub limit: u32,
}

impl<T> PaginatedResponse<T> {
    /// Check if there are more pages available.
    pub fn has_more(&self) -> bool {
        self.offset + self.limit < self.total
    }

    /// Get the next pagination parameters.
    pub fn next_page(&self) -> Option<Pagination> {
        if self.has_more() {
            Some(Pagination {
                offset: Some(self.offset + self.limit),
                limit: Some(self.limit),
            })
        } else {
            None
        }
    }
}
