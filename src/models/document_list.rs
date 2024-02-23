use serde::{Deserialize, Serialize};

use super::document::Document;

/// Document List
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentList {
    /// Total number of document documents that matched your query.
    total: u64,
    /// List of document.
    documents: Vec<Document>,
}
