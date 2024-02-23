use serde::{Deserialize, Serialize};

use super::language::Language;

/// Languages List
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageList {
    /// Total number of languages documents that matched your query.
    total: u64,
    /// List of languages.
    languages: Vec<Language>,
}
