use serde::{Deserialize, Serialize};

/// AlgoScryptModified
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoScryptModified {
    /// algo type.
    #[serde(rename = "type")]
    pub algo_type: String,

    /// Salt used to compute hash.
    pub salt: String,

    /// Separator used to compute hash.
    #[serde(rename = "saltSeparator")]
    pub salt_separator: String,

    /// Key used to compute hash.
    #[serde(rename = "signerKey")]
    pub signer_key: String,
}
