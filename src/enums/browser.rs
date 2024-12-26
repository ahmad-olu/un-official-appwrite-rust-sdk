use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum Browser {
    #[serde(rename = "aa")]
    #[default]
    AvantBrowser,
    #[serde(rename = "an")]
    AndroidWebViewBeta,
    #[serde(rename = "ch")]
    GoogleChrome,
    #[serde(rename = "ci")]
    GoogleChromeIOS,
    #[serde(rename = "cm")]
    GoogleChromeMobile,
    #[serde(rename = "cr")]
    Chromium,
    #[serde(rename = "ff")]
    MozillaFirefox,
    #[serde(rename = "sf")]
    Safari,
    #[serde(rename = "mf")]
    MobileSafari,
    #[serde(rename = "ps")]
    MicrosoftEdge,
    #[serde(rename = "oi")]
    MicrosoftEdgeIOS,
    #[serde(rename = "om")]
    OperaMini,
    #[serde(rename = "op")]
    Opera,
    #[serde(rename = "on")]
    OperaNext,
}

impl Browser {
    /// Converts the enum to its serialized string representation
    pub fn as_serialized(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| panic!("Failed to serialize Browser: {:?}", self))
            .trim_matches('"') // Remove surrounding quotes from the JSON string
            .to_string()
    }
}
