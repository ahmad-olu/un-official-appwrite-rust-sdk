use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageFormat {
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
}
