use serde::{Deserialize, Serialize};

/// AlgoMD5
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoMd5 {
    /// algo type.
    #[serde(rename = "type")]
    pub algo_type: String,
}
