use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum OAuthProvider {
    #[default]
    #[serde(rename = "amazon")]
    Amazon,
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "auth0")]
    Auth0,
    #[serde(rename = "authentik")]
    Authentik,
    #[serde(rename = "autodesk")]
    Autodesk,
    #[serde(rename = "bitbucket")]
    Bitbucket,
    #[serde(rename = "bitly")]
    Bitly,
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "dailymotion")]
    Dailymotion,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "disqus")]
    Disqus,
    #[serde(rename = "dropbox")]
    Dropbox,
    #[serde(rename = "etsy")]
    Etsy,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "linkedin")]
    Linkedin,
    #[serde(rename = "microsoft")]
    Microsoft,
    #[serde(rename = "notion")]
    Notion,
    #[serde(rename = "oidc")]
    Oidc,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "paypalSandbox")]
    PaypalSandbox,
    #[serde(rename = "podio")]
    Podio,
    #[serde(rename = "salesforce")]
    Salesforce,
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "stripe")]
    Stripe,
    #[serde(rename = "tradeshift")]
    Tradeshift,
    #[serde(rename = "tradeshiftBox")]
    TradeshiftBox,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "wordpress")]
    Wordpress,
    #[serde(rename = "yahoo")]
    Yahoo,
    #[serde(rename = "yammer")]
    Yammer,
    #[serde(rename = "yandex")]
    Yandex,
    #[serde(rename = "zoho")]
    Zoho,
    #[serde(rename = "zoom")]
    Zoom,
    #[serde(rename = "mock")]
    Mock,
}
