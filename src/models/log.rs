use serde::{Deserialize, Serialize};

/// Log
#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    /// Event name.
    event: String,

    /// User ID.
    #[serde(rename = "userId")]
    user_id: String,

    /// User Email.
    #[serde(rename = "userEmail")]
    user_email: String,

    /// User Name.
    #[serde(rename = "userName")]
    user_name: String,

    /// API mode when event triggered.
    mode: String,

    /// IP session in use when the session was created.
    ip: String,

    /// Log creation date in ISO 8601 format.
    #[serde(rename = "teamName")]
    time: String,

    /// Operating system code name. View list of [available options](https://github.com/appwrite/appwrite/blob/master/docs/lists/os.json).
    #[serde(rename = "osCode")]
    os_code: String,

    /// Operating system name.
    #[serde(rename = "osName")]
    os_name: String,

    /// Operating system version.
    #[serde(rename = "osVersion")]
    os_version: String,

    /// Client type.
    #[serde(rename = "clientType")]
    client_type: String,

    /// Client code name. View list of [available options](https://github.com/appwrite/appwrite/blob/master/docs/lists/clients.json).
    #[serde(rename = "clientCode")]
    client_code: String,

    /// Client name.
    #[serde(rename = "clientName")]
    client_name: String,

    /// Client version.
    #[serde(rename = "clientVersion")]
    client_version: String,

    /// Client engine name.
    #[serde(rename = "clientEngine")]
    client_engine: String,

    /// Client engine name.
    #[serde(rename = "clientEngineVersion")]
    client_engine_version: String,

    /// Device name.
    #[serde(rename = "deviceName")]
    device_name: String,

    /// Device brand name.
    #[serde(rename = "deviceBrand")]
    device_brand: String,

    /// Device model name.
    #[serde(rename = "deviceModel")]
    device_model: String,

    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode")]
    country_code: String,

    /// Country name.
    #[serde(rename = "countryName")]
    country_name: String,
}
