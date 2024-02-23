use serde::{Deserialize, Serialize};

/// Identity
#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    /// Identity ID.
    #[serde(rename = "$id")]
    id: String,

    /// Identity creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Identity update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// User ID.
    #[serde(rename = "userId")]
    user_id: String,

    /// Identity Provider.
    provider: String,

    /// ID of the User in the Identity Provider.
    #[serde(rename = "providerUid")]
    provider_uid: String,

    /// Email of the User in the Identity Provider.
    #[serde(rename = "providerEmail")]
    provider_email: String,

    /// Identity Provider Access Token.
    #[serde(rename = "providerAccessToken")]
    provider_access_token: String,

    /// The date of when the access token expires in ISO 8601 format.
    #[serde(rename = "providerAccessTokenExpiry")]
    provider_access_token_expiry: String,

    /// Identity Provider Refresh Token.
    #[serde(rename = "providerRefreshToken")]
    provider_refresh_token: String,
}
