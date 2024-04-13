use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RelationMutate {
    #[serde(rename = "cascade")]
    Cascade,
    #[serde(rename = "restrict")]
    Restrict,
    #[serde(rename = "setNull")]
    SetNull,
}
