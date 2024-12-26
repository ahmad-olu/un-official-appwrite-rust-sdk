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

impl ImageFormat {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize ImageFormat: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
