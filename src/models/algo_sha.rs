use serde::{Deserialize, Serialize};

/// AlgoSHA
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct AlgoSha {
    /// algo type.
    #[serde(rename = "type")]
    pub algo_type: String,
}
