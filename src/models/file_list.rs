use serde::{Deserialize, Serialize};

use super::file::File;

/// Files List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct FileList {
    /// Total number of files documents that matched your query.
    pub total: u64,
    /// List of files.
    pub files: Vec<File>,
}
