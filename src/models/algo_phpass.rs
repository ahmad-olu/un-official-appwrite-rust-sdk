use serde::{Deserialize, Serialize};

/// AlgoPHPASS
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct AlgoPhpass {
    /// algo type.
    #[serde(rename = "type")]
    pub algo_type: String,
}
