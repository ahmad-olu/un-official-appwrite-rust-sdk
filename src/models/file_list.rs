use serde::{Deserialize, Serialize};

use super::file::File;

/// Files List
#[derive(Debug, Serialize, Deserialize)]
pub struct FileList {
    /// Total number of files documents that matched your query.
    total: u64,
    /// List of files.
    files: Vec<File>,
}
