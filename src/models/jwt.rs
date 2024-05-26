use serde::{Deserialize, Serialize};

/// JWT
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct JWT {
    /// JWT encoded string.
    pub jwt: String,
}
