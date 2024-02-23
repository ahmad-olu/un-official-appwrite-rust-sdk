use serde::{Deserialize, Serialize};

/// AlgoScryptModified
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoScryptModified {
    /// algo type.
    #[serde(rename = "type")]
    algo_type: String,

    /// Salt used to compute hash.
    salt: String,

    /// Separator used to compute hash.
    #[serde(rename = "saltSeparator")]
    salt_separator: String,

    /// Key used to compute hash.
    #[serde(rename = "signerKey")]
    signer_key: String,
}
