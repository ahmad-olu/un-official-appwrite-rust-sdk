use serde::{Deserialize, Serialize};

/// Headers
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Headers {
    /// Header name.
    pub name: String,
    /// Header value.
    pub value: String,
}
