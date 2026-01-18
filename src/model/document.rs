//! Document-related models.

use serde::{Deserialize, Serialize};

use super::{Id, Timestamp};

/// Document type/format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentType {
    /// Word document (.docx)
    Docx,
    /// PDF document (.pdf)
    Pdf,
    /// Excel spreadsheet (.xlsx)
    Xlsx,
    /// PowerPoint presentation (.pptx)
    Pptx,
    /// SVG image (.svg)
    Svg,
    /// JPEG image (.jpg, .jpeg)
    Jpeg,
    /// PNG image (.png)
    Png,
    /// JSON file (.json)
    Json,
    /// XML file (.xml)
    Xml,
    /// Plain text (.txt)
    Text,
    /// Other/unknown format
    Other,
}

impl DocumentType {
    /// Get the file extension for this document type.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Docx => "docx",
            Self::Pdf => "pdf",
            Self::Xlsx => "xlsx",
            Self::Pptx => "pptx",
            Self::Svg => "svg",
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Json => "json",
            Self::Xml => "xml",
            Self::Text => "txt",
            Self::Other => "bin",
        }
    }

    /// Get the MIME type for this document type.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            Self::Pdf => "application/pdf",
            Self::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            Self::Pptx => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            }
            Self::Svg => "image/svg+xml",
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Json => "application/json",
            Self::Xml => "application/xml",
            Self::Text => "text/plain",
            Self::Other => "application/octet-stream",
        }
    }

    /// Detect document type from file extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().trim_start_matches('.') {
            "docx" | "doc" => Self::Docx,
            "pdf" => Self::Pdf,
            "xlsx" | "xls" => Self::Xlsx,
            "pptx" | "ppt" => Self::Pptx,
            "svg" | "svgz" => Self::Svg,
            "jpg" | "jpeg" => Self::Jpeg,
            "png" => Self::Png,
            "json" => Self::Json,
            "xml" => Self::Xml,
            "txt" | "text" => Self::Text,
            _ => Self::Other,
        }
    }
}

/// A document stored in Nvisy.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Document {
    /// Unique identifier.
    pub id: Id,
    /// Document name/title.
    pub name: String,
    /// Document type.
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    /// File size in bytes.
    pub size: u64,
    /// ID of the workspace this document belongs to.
    pub workspace_id: Id,
    /// ID of the user who uploaded this document.
    pub uploaded_by: Id,
    /// When the document was created.
    pub created_at: Timestamp,
    /// When the document was last updated.
    pub updated_at: Timestamp,
}

/// Request to create/upload a new document.
#[derive(Debug, Clone, Serialize)]
pub struct CreateDocumentRequest {
    /// Document name/title.
    pub name: String,
    /// Document type.
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    /// ID of the workspace to upload to.
    pub workspace_id: Id,
}

/// Request to update document metadata.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDocumentRequest {
    /// New document name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New workspace ID (move document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Id>,
}

/// Document version information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentVersion {
    /// Version number.
    pub version: u32,
    /// File size in bytes.
    pub size: u64,
    /// ID of the user who created this version.
    pub created_by: Id,
    /// When this version was created.
    pub created_at: Timestamp,
}
