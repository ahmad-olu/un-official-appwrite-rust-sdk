use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Compression {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "zstd")]
    Zstd,
}
