use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Browser {
    #[serde(rename = "aa")]
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
