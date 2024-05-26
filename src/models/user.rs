use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{preferences::Preferences, target::Target};

/// User
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct User {
    /// User ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// User creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// User update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// User name.
    pub name: String,

    /// Hashed user password.
    pub password: Option<String>,

    /// Password hashing algorithm.
    pub hash: Option<String>,

    /// Password hashing algorithm configuration.
    #[serde(rename = "hashOptions")]
    pub hash_options: Value,

    /// User registration date in ISO 8601 format.
    pub registration: String,

    /// User status. Pass `true` for enabled and `false` for disabled.
    pub status: bool,

    /// Labels for the user.
    pub labels: Vec<Value>,

    /// Password update time in ISO 8601 format.
    #[serde(rename = "passwordUpdate")]
    pub password_update: String,

    /// User email address.
    pub email: String,

    /// User phone number in E.164 format.
    pub phone: String,

    /// Email verification status.
    #[serde(rename = "emailVerification")]
    pub email_verification: bool,

    /// Phone verification status.
    #[serde(rename = "phoneVerification")]
    pub phone_verification: bool,

    /// Multi factor authentication status.
    pub mfa: Option<bool>,

    /// User preferences as a key-value object
    #[serde(rename = "prefs")]
    pub preferences: Preferences,

    /// A user-owned message receiver. A single user may have multiple e.g. emails, phones, and a browser. Each target is registered with a single provider.
    pub targets: Option<Vec<Target>>,

    /// Most recent access date in ISO 8601 format. This attribute is only updated again after 24 hours.
    #[serde(rename = "accessedAt")]
    pub accessed_at: String,
}
