use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Session
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// Session ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Session creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Session expiration date in ISO 8601 format.
    pub expire: String,

    /// Session Provider.
    pub provider: String,

    /// Session Provider User ID.
    #[serde(rename = "providerUid")]
    pub provider_uid: String,

    /// Session Provider Access Token.
    #[serde(rename = "providerAccessToken")]
    pub provider_access_token: String,

    /// The date of when the access token expires in ISO 8601 format.
    #[serde(rename = "providerAccessTokenExpiry")]
    pub provider_access_token_expiry: String,

    /// Session Provider Refresh Token.
    #[serde(rename = "providerRefreshToken")]
    pub provider_refresh_token: String,

    /// IP in use when the session was created.
    pub ip: String,

    /// Operating system code name. View list of [available options](https://github.com/appwrite/appwrite/blob/master/docs/lists/os.json).
    #[serde(rename = "osCode")]
    pub os_code: String,

    /// Operating system name.
    #[serde(rename = "osName")]
    pub os_name: String,

    /// Operating system version.
    #[serde(rename = "osVersion")]
    pub os_version: String,

    /// Client type.
    #[serde(rename = "clientType")]
    pub client_type: String,

    /// Client code name. View list of [available options](https://github.com/appwrite/appwrite/blob/master/docs/lists/clients.json).
    #[serde(rename = "clientCode")]
    pub client_code: String,

    /// Client name.
    #[serde(rename = "clientName")]
    pub client_name: String,

    /// Client version.
    #[serde(rename = "clientVersion")]
    pub client_version: String,

    /// Client engine name.
    #[serde(rename = "clientEngine")]
    pub client_engine: String,

    /// Client engine version.
    #[serde(rename = "clientEngineVersion")]
    pub client_engine_version: String,

    /// Device name.
    #[serde(rename = "deviceName")]
    pub device_name: String,

    /// Device brand name.
    #[serde(rename = "deviceBrand")]
    pub device_brand: String,

    /// Device model name.
    #[serde(rename = "deviceModel")]
    pub device_model: String,

    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode")]
    pub country_code: String,

    /// Country name.
    #[serde(rename = "countryName")]
    pub country_name: String,

    /// Returns true if this is the current user session.
    pub current: bool,

    /// Returns a list of active session factors.
    pub factors: Vec<Value>,

    /// Secret used to authenticate the user. Only included if the request was made with an API key
    pub secret: String,

    /// Most recent date in ISO 8601 format when the session successfully passed MFA challenge.
    #[serde(rename = "mfaUpdatedAt")]
    pub mfa_updated_at: String,
}
