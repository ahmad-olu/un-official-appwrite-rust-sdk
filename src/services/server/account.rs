use reqwest::header;
use serde_json::Value;

use crate::{
    client::Client,
    enums::HttpMethod,
    error::Error,
    models::{
        identity_list::IdentityList, log_list::LogList, preferences::Preferences, session::Session,
        session_list::SessionList, token::Token, user::User,
    },
};

/// ACCOUNT
/// The Account service allows you to authenticate and manage a user account.
pub struct Account;

impl Account {
    /// Get account
    ///
    /// Get the currently logged in user.
    async fn get(client: &Client) -> Result<User, Error> {
        const API_PATH: &str = "/account";

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    //Create account
    //
    // Use this endpoint to allow a new user to register a new account in your
    // project. After the user registration completes successfully, you can use
    // the
    // [/account/verfication](https://appwrite.io/docs/references/cloud/client-web/account#createVerification)
    // route to start verifying the user email address. To allow the new user to
    // login to their new account, you need to create a new [account
    // session](https://appwrite.io/docs/references/cloud/client-web/account#createEmailSession).
    async fn create(
        client: &Client,
        user_id: &str,
        email: &str,
        password: &str,
        name: Option<&str>,
    ) -> Result<User, Error> {
        const API_PATH: &str = "/account";

        let api_params = serde_json::json!({
            "userId":user_id,
            "email":email,
            "password":password,
            "name": name,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update email
    ///
    /// Update currently logged in user account email address. After changing user
    /// address, the user confirmation status will get reset. A new confirmation
    /// email is not sent automatically however you can use the send confirmation
    /// email endpoint again to send the confirmation email. For security measures,
    /// user password is required to complete this request.
    /// This endpoint can also be used to convert an anonymous account to a normal
    /// one, by passing an email address and a new password.
    ///
    async fn update_email(client: &Client, email: &str, password: &str) -> Result<User, Error> {
        const API_PATH: &str = "/account/email";

        let api_params = serde_json::json!({
            "email":email,
            "password":password,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::PATCH, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List Identities
    ///
    /// Get the list of identities for the currently logged in user.
    async fn list_identities(
        client: &Client,
        queries: Option<&str>,
    ) -> Result<IdentityList, Error> {
        const API_PATH: &str = "/account/identities";

        let api_params = serde_json::json!({
            "queries":queries,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete Identity
    ///
    /// Delete an identity by its unique ID.
    async fn delete_identity(client: &Client, identity_id: &str) -> Result<(), Error> {
        let api_path: String = r"/account/identities/{identityId}"
            .to_owned()
            .replace("{identityId}", identity_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(())
    }

    /// List logs
    ///
    /// Get the list of latest security activity logs for the currently logged in
    /// user. Each log returns user IP address, location and date and time of log.
    async fn list_logs(client: &Client, queries: Option<Vec<&str>>) -> Result<LogList, Error> {
        const API_PATH: &str = "/account/logs";

        let api_params = serde_json::json!({
            "queries":queries,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update name
    ///
    /// Update currently logged in user account name.
    async fn update_name(client: &Client, name: &str) -> Result<User, Error> {
        const API_PATH: &str = "/account/name";

        let api_params = serde_json::json!({
            "name": name
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::PATCH, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update password
    ///
    /// Update currently logged in user password. For validation, user is required
    /// to pass in the new password, and the old password. For users created with
    /// OAuth, Team Invites and Magic URL, oldPassword is optional.
    async fn update_password(
        client: &Client,
        password: &str,
        old_password: &str,
    ) -> Result<User, Error> {
        const API_PATH: &str = "/account/name";

        let api_params = serde_json::json!({
            "password": password,
            "oldPassword":old_password,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::PATCH, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update phone
    ///
    /// Update the currently logged in user's phone number. After updating the
    /// phone number, the phone verification status will be reset. A confirmation
    /// SMS is not sent automatically, however you can use the [POST
    /// /account/verification/phone](https://appwrite.io/docs/references/cloud/client-web/account#createPhoneVerification)
    /// endpoint to send a confirmation SMS.
    async fn update_phone(client: &Client, phone: &str, password: &str) -> Result<User, Error> {
        const API_PATH: &str = "/account/name";

        let api_params = serde_json::json!({
            "phone": phone,
            "password":password,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::PATCH, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get account preferences
    ///
    /// Get the preferences as a key-value object for the currently logged in user.
    async fn get_preference(client: &Client) -> Result<Preferences, Error> {
        const API_PATH: &str = "/account/prefs";

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update preferences
    ///
    /// Update currently logged in user account preferences. The object you pass is
    /// stored as is, and replaces any previous value. The maximum allowed prefs
    /// size is 64kB and throws error if exceeded.
    async fn update_preference(client: &Client, preference: Value) -> Result<User, Error> {
        const API_PATH: &str = "/account/prefs";

        let api_params = serde_json::json!({
            "prefs": preference,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::PATCH, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create password recovery
    ///
    /// Sends the user an email with a temporary secret key for password reset.
    /// When the user clicks the confirmation link he is redirected back to your
    /// app password reset URL with the secret key and email address values
    /// attached to the URL query string. Use the query string params to submit a
    /// request to the [PUT
    /// /account/recovery](https://appwrite.io/docs/references/cloud/client-web/account#updateRecovery)
    /// endpoint to complete the process. The verification link sent to the user's
    /// email address is valid for 1 hour.
    async fn create_recovery(client: &Client, email: &str, url: &str) -> Result<Token, Error> {
        const API_PATH: &str = "/account/recovery";

        let api_params = serde_json::json!({
            "email":email,
            "url":url
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create password recovery (confirmation)
    ///
    /// Use this endpoint to complete the user account password reset. Both the
    /// **userId** and **secret** arguments will be passed as query parameters to
    /// the redirect URL you have provided when sending your request to the [POST
    /// /account/recovery](https://appwrite.io/docs/references/cloud/client-web/account#createRecovery)
    /// endpoint.
    ///
    /// Please note that in order to avoid a [Redirect
    /// Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md)
    /// the only valid redirect URLs are the ones from domains you have set when
    /// adding your platforms in the console interface.
    async fn update_recovery(
        client: &Client,
        user_id: &str,
        secret: &str,
        password: &str,
        password_again: &str,
    ) -> Result<Token, Error> {
        const API_PATH: &str = "/account/recovery";

        let api_params = serde_json::json!({
            "userId":user_id,
            "secret": secret,
            "password":password,
            "passwordAgain":password_again,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::PUT, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List sessions
    ///
    /// Get the list of active sessions across different devices for the currently
    /// logged in user.
    async fn list_sessions(client: &Client) -> Result<SessionList, Error> {
        const API_PATH: &str = "/account/sessions";

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete sessions
    ///
    /// Delete all sessions from the user account and remove any sessions cookies
    /// from the end client.
    async fn delete_sessions(client: &Client) -> Result<(), Error> {
        const API_PATH: &str = "/account/sessions";

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let _res = client
            .call(HttpMethod::DELETE, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(())
    }

    /// Get session
    ///
    /// Use this endpoint to get a logged in user's session using a Session ID.
    /// Inputting 'current' will return the current session being used.
    async fn get_session(client: &Client, session_id: &str) -> Result<Session, Error> {
        let api_path: String = r"/account/sessions/{sessionId}"
            .to_owned()
            .replace("{sessionId}", session_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::GET,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update OAuth session (refresh tokens)
    ///
    /// Access tokens have limited lifespan and expire to mitigate security risks.
    /// If session was created using an OAuth provider, this route can be used to
    /// "refresh" the access token.
    async fn update_session(client: &Client, session_id: &str) -> Result<Session, Error> {
        let api_path: String = r"/account/sessions/{sessionId}"
            .to_owned()
            .replace("{sessionId}", session_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete session
    ///
    /// Logout the user. Use 'current' as the session ID to logout on this device,
    /// use a session ID to logout on another device. If you're looking to logout
    /// the user on all devices, use [Delete
    /// Sessions](https://appwrite.io/docs/references/cloud/client-web/account#deleteSessions)
    /// instead.
    async fn delete_session(client: &Client, session_id: &str) -> Result<(), Error> {
        let api_path: String = r"/account/sessions/{sessionId}"
            .to_owned()
            .replace("{sessionId}", session_id);

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await;

        Ok(())
    }

    /// Update status
    ///
    /// Block the currently logged in user account. Behind the scene, the user
    /// record is not deleted but permanently blocked from any access. To
    /// completely delete a user, use the Users API instead.
    pub async fn update_status(client: &Client) -> Result<User, Error> {
        const API_PATH: &str = "/account/status";

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::DELETE, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create email verification
    ///
    /// Use this endpoint to send a verification message to your user email address
    /// to confirm they are the valid owners of that address. Both the **userId**
    /// and **secret** arguments will be passed as query parameters to the URL you
    /// have provided to be attached to the verification email. The provided URL
    /// should redirect the user back to your app and allow you to complete the
    /// verification process by verifying both the **userId** and **secret**
    /// parameters. Learn more about how to [complete the verification
    /// process](https://appwrite.io/docs/references/cloud/client-web/account#updateVerification).
    /// The verification link sent to the user's email address is valid for 7 days.
    ///
    /// Please note that in order to avoid a [Redirect
    /// Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md),
    /// the only valid redirect URLs are the ones from domains you have set when
    /// adding your platforms in the console interface.
    ///
    async fn create_verification(client: &Client, url: &str) -> Result<Token, Error> {
        const API_PATH: &str = "/account/verification";

        let api_params = serde_json::json!({
            "url":url
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create email verification (confirmation)
    ///
    /// Use this endpoint to complete the user email verification process. Use both
    /// the **userId** and **secret** parameters that were attached to your app URL
    /// to verify the user email ownership. If confirmed this route will return a
    /// 200 status code.
    async fn update_verification(
        client: &Client,
        user_id: &str,
        secret: &str,
    ) -> Result<Token, Error> {
        const API_PATH: &str = "/account/verification";

        let api_params = serde_json::json!({
            "userId": user_id,
            "secret": secret
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::PUT, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create phone verification
    ///
    /// Use this endpoint to send a verification SMS to the currently logged in
    /// user. This endpoint is meant for use after updating a user's phone number
    /// using the
    /// [accountUpdatePhone](https://appwrite.io/docs/references/cloud/client-web/account#updatePhone)
    /// endpoint. Learn more about how to [complete the verification
    /// process](https://appwrite.io/docs/references/cloud/client-web/account#updatePhoneVerification).
    /// The verification code sent to the user's phone number is valid for 15
    /// minutes.
    async fn create_phone_verification(client: &Client) -> Result<Token, Error> {
        const API_PATH: &str = "/account/verification/phone";

        let api_params = serde_json::json!({});

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create phone verification (confirmation)
    ///
    /// Use this endpoint to complete the user phone verification process. Use the
    /// **userId** and **secret** that were sent to your user's phone number to
    /// verify the user email ownership. If confirmed this route will return a 200
    /// status code.
    async fn update_phone_verification(
        client: &Client,
        user_id: &str,
        secret: &str,
    ) -> Result<Token, Error> {
        const API_PATH: &str = "/account/verification/phone";

        let api_params = serde_json::json!({
            "userId":user_id,
            "secret": secret
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::PUT, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }
}
