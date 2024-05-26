use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Preferences
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Preferences {
    #[serde(flatten)]
    pub data: Map<String, Value>,
}
