use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum RelationMutate {
    #[default]
    #[serde(rename = "cascade")]
    Cascade,
    #[serde(rename = "restrict")]
    Restrict,
    #[serde(rename = "setNull")]
    SetNull,
}
