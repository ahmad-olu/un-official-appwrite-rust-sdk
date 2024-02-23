use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::preferences::Preferences;

/// User
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// User ID.
    #[serde(rename = "$id")]
    id: String,

    /// User creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// User update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// User name.
    name: String,

    /// Hashed user password.
    password: Option<String>,

    /// Password hashing algorithm.
    hash: Option<String>,

    /// Password hashing algorithm configuration.
    #[serde(rename = "hashOptions")]
    hash_options: Value,

    /// User registration date in ISO 8601 format.
    registration: String,

    /// User status. Pass `true` for enabled and `false` for disabled.
    status: bool,

    /// Labels for the user.
    ///! this should be a list of something /
    labels: Value,

    /// Password update time in ISO 8601 format.
    #[serde(rename = "passwordUpdate")]
    password_update: String,

    /// User email address.
    email: String,

    /// User phone number in E.164 format.
    phone: String,

    /// Email verification status.
    #[serde(rename = "emailVerification")]
    email_verification: bool,

    /// Phone verification status.
    #[serde(rename = "phoneVerification")]
    phone_verification: bool,

    /// User preferences as a key-value object
    #[serde(rename = "prefs")]
    preferences: Preferences,

    /// Most recent access date in ISO 8601 format. This attribute is only updated again after 24 hours.
    #[serde(rename = "accessedAt")]
    accessed_at: String,
}
