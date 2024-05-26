use serde::{Deserialize, Serialize};

/// Log
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Log {
    /// Event name.
    pub event: String,

    /// User ID.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// User Email.
    #[serde(rename = "userEmail")]
    pub user_email: String,

    /// User Name.
    #[serde(rename = "userName")]
    pub user_name: String,

    /// API mode when event triggered.
    pub mode: String,

    /// IP session in use when the session was created.
    pub ip: String,

    /// Log creation date in ISO 8601 format.
    #[serde(rename = "teamName")]
    pub time: String,

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

    /// Client engine name.
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
}
