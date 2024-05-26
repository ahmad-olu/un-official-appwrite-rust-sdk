use serde::{Deserialize, Serialize};

/// Identity
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Identity {
    /// Identity ID.
    #[serde(rename = "$id")]
   pub     id: String,

    /// Identity creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
   pub     created_at: String,

    /// Identity update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
   pub     updated_at: String,

    /// User ID.
    #[serde(rename = "userId")]
   pub     user_id: String,

    /// Identity Provider.
   pub     provider: String,

    /// ID of the User in the Identity Provider.
    #[serde(rename = "providerUid")]
   pub     provider_uid: String,

    /// Email of the User in the Identity Provider.
    #[serde(rename = "providerEmail")]
   pub     provider_email: String,

    /// Identity Provider Access Token.
    #[serde(rename = "providerAccessToken")]
   pub     provider_access_token: String,

    /// The date of when the access token expires in ISO 8601 format.
    #[serde(rename = "providerAccessTokenExpiry")]
   pub     provider_access_token_expiry: String,

    /// Identity Provider Refresh Token.
    #[serde(rename = "providerRefreshToken")]
   pub     provider_refresh_token: String,
}
