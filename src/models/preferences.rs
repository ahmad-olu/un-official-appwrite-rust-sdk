use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Preferences
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Preferences {
    #[serde(flatten)]
    pub data: HashMap<String, Value>,
}
