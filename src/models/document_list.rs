use serde::{Deserialize, Serialize};

use super::document::Document;

/// Document List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct DocumentList {
    /// Total number of document documents that matched your query.
    pub total: u64,
    /// List of document.
    pub documents: Vec<Document>,
}
