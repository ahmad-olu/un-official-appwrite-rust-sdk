use serde::{Deserialize, Serialize};

use super::locale_code::LocaleCode;

/// Locale codes List
#[derive(Debug, Serialize, Deserialize)]
pub struct LocaleCodeList {
    /// Total number of localeCodes documents that matched your query.
    total: u64,
    /// List of localeCodes.
    #[serde(rename = "localeCodes")]
    locale_codes: Vec<LocaleCode>,
}
