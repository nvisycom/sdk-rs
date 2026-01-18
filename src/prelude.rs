//! Convenient re-exports for common types.
//!
//! This module provides a single import for commonly used types.
//!
//! ```ignore
//! use nvisy_studio_client::prelude::*;
//! ```

pub use crate::client::{NvisyClient, NvisyConfig, NvisyConfigBuilder};
pub use crate::error::{Error, Result};
pub use crate::model::{
    CreateDocumentRequest, CreateWorkspaceRequest, Document, DocumentType, DocumentVersion, Id,
    PaginatedResponse, Pagination, Timestamp, UpdateDocumentRequest, UpdateWorkspaceRequest,
    Workspace,
};
pub use crate::service::{DocumentService, WorkspaceService};
