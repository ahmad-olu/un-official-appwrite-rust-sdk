use serde::{Deserialize, Serialize};

use super::locale_code::LocaleCode;

/// Locale codes List
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct LocaleCodeList {
    /// Total number of localeCodes documents that matched your query.
    pub total: u64,
    /// List of localeCodes.
    #[serde(rename = "localeCodes")]
    pub locale_codes: Vec<LocaleCode>,
}
