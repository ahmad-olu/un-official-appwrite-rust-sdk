use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum IndexType {
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "fulltext")]
    Fulltext,
    #[serde(rename = "unique")]
    Unique,
}
