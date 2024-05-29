//!  # ACCOUNT
//!
//! The Account service allows you to authenticate and manage a user account.

use serde_json::{json, Value};

use crate::{
    api_params, app_json_header,
    client::Client,
    enumm::HttpMethod,
    enums::{
        authentication_factor::AuthenticationFactor, authentication_type::AuthenticationType,
        o_auth_provider::OAuthProvider,
    },
    error::Error,
    models::{
        identity_list::IdentityList, jwt::JWT, log_list::LogList, mfa_challenge::MfaChallenge,
        mfa_factors::MfaFactors, mfa_recovery_codes::MfaRecoveryCodes, mfa_type::MfaType,
        preferences::Preferences, session::Session, session_list::SessionList, token::Token,
        user::User,
    },
    utils::get_content_header_value,
};

pub struct Account;

impl Account {
    /// Get account
    ///
    /// Get the currently logged in user.
    pub async fn get(client: &Client) -> Result<User, Error> {
        const API_PATH: &str = "/account";

        let api_params = api_params! {};

        let api_headers = app_json_header!();

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
    // [/account/verification](https://appwrite.io/docs/references/cloud/client-web/account#createVerification)
    // route to start verifying the user email address. To allow the new user to
    // login to their new account, you need to create a new [account
    // session](https://appwrite.io/docs/references/cloud/client-web/account#createEmailSession).
    pub async fn create(
        client: &Client,
        user_id: &str,
        email: &str,
        password: &str,
        name: Option<&str>,
    ) -> Result<User, Error> {
        const API_PATH: &str = "/account";

        let api_params = api_params! {
           "userId" => Some(user_id),
            "email" => Some(email),
            "password" => Some(password),
            "name"=> name,
        };

        let api_headers = app_json_header!();

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
    pub async fn update_email(client: &Client, email: &str, password: &str) -> Result<User, Error> {
        const API_PATH: &str = "/account/email";

        let api_params = api_params! {
            "email" => Some(email),
            "password" => Some(password),
        };

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PATCH, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List Identities
    ///
    /// Get the list of identities for the currently logged in user.
    pub async fn list_identities(
        client: &Client,
        queries: Option<String>,
    ) -> Result<IdentityList, Error> {
        const API_PATH: &str = "/account/identities";

        let api_params = api_params! {
            "queries"=> queries,
        };

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete Identity
    ///
    /// Delete an identity by its unique ID.
    pub async fn delete_identity(client: &Client, identity_id: &str) -> Result<(), Error> {
        let api_path: String = r"/account/identities/{identityId}"
            .to_owned()
            .replace("{identityId}", identity_id);

        let api_params = api_params! {};

        let api_headers = app_json_header!();

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

    /// Create JWT
    ///
    /// Use this endpoint to create a JSON Web Token. You can use the resulting JWT
    /// to authenticate on behalf of the current user when working with the
    /// Appwrite server-side API and SDKs. The JWT secret is valid for 15 minutes
    /// from its creation and will be invalid if the user will logout in that time
    /// frame.
    pub async fn create_jwt(client: &Client) -> Result<JWT, Error> {
        let api_path = "/account/jwt";

        let api_params = api_params! {};

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List logs
    ///
    /// Get the list of latest security activity logs for the currently logged in
    /// user. Each log returns user IP address, location and date and time of log.
    pub async fn list_logs(
        client: &Client,
        queries: Option<Vec<String>>,
    ) -> Result<LogList, Error> {
        const API_PATH: &str = "/account/logs";

        let api_params = api_params! {
            "queries"=> queries,
        };

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update MFA
    ///
    /// Enable or disable MFA on an account.
    pub async fn update_mfa(client: &Client, mfa: bool) -> Result<User, Error> {
        let api_path = "/account/mfa";

        let api_params = api_params! {
            "mfa"=> Some(mfa),
        };

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PATCH, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Add Authenticator
    ///
    /// Add an authenticator app to be used as an MFA factor. Verify the
    /// authenticator using the [verify
    /// authenticator](/docs/references/cloud/client-web/account#verifyAuthenticator)
    /// method.
    pub async fn create_mfa_authenticator(
        client: &Client,
        x_type: AuthenticationType,
    ) -> Result<MfaType, Error> {
        let api_path = format!("/account/mfa/authenticators/{}", json!(x_type));

        let api_params = api_params! {};

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Verify Authenticator
    ///
    /// Verify an authenticator app after adding it using the [add
    /// authenticator](/docs/references/cloud/client-web/account#addAuthenticator)
    /// method.
    pub async fn update_mfa_authenticator(
        client: &Client,
        x_type: AuthenticationType,
        otp: &str,
    ) -> Result<User, Error> {
        let api_path = format!("/account/mfa/authenticators/{}", json!(x_type));

        let api_params = api_params! {
            "otp"=> Some(otp),
        };

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PUT,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete Authenticator
    ///
    /// Delete an authenticator for a user by ID.
    pub async fn delete_mfa_authenticator(
        client: &Client,
        x_type: AuthenticationType,
        otp: &str,
    ) -> Result<User, Error> {
        let api_path = format!("/account/mfa/authenticators/{}", json!(x_type));

        let api_params = api_params! {
            "otp"=> Some(otp),
        };

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create 2FA Challenge
    ///
    /// Begin the process of MFA verification after sign-in. Finish the flow with
    /// [updateMfaChallenge](/docs/references/cloud/client-web/account#updateMfaChallenge)
    /// method.
    pub async fn create_mfa_challenge(
        client: &Client,
        factor: AuthenticationFactor,
    ) -> Result<MfaChallenge, Error> {
        let api_path = "/account/mfa/challenge";

        let api_params = api_params! {
            "factor"=> Some(factor),
        };

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create MFA Challenge (confirmation)
    ///
    /// Complete the MFA challenge by providing the one-time password. Finish the
    /// process of MFA verification by providing the one-time password. To begin
    /// the flow, use
    /// [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge)
    /// method.
    pub async fn update_mfa_challenge(
        client: &Client,
        challenge_id: &str,
        otp: &str,
    ) -> Result<(), Error> {
        let api_path = "/account/mfa/challenge";

        let api_params = api_params! {
            "challengeId"=> Some(challenge_id),
            "otp" =>Some(otp)
        };

        let api_headers = app_json_header!();

        let _res = client
            .call(HttpMethod::PUT, api_path, api_headers, &api_params, None)
            .await?;

        Ok(())
    }

    /// List Factors
    ///
    /// List the factors available on the account to be used as a MFA challenge.
    pub async fn list_mfa_factors(client: &Client) -> Result<MfaFactors, Error> {
        let api_path = "/account/mfa/factors";

        let api_params = api_params! {};

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get MFA Recovery Codes
    ///
    /// Get recovery codes that can be used as backup for MFA flow. Before getting
    /// codes, they must be generated using
    /// [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes)
    /// method. An OTP challenge is required to read recovery codes.
    pub async fn get_mfa_recovery_codes(client: &Client) -> Result<MfaRecoveryCodes, Error> {
        let api_path = "/account/mfa/recovery-codes";

        let api_params = api_params! {};

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create MFA Recovery Codes
    ///
    /// Generate recovery codes as backup for MFA flow. It's recommended to
    /// generate and show then immediately after user successfully adds their
    /// authenticator. Recovery codes can be used as a MFA verification type in
    /// [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge)
    /// method.
    pub async fn create_mfa_recovery_codes(client: &Client) -> Result<MfaRecoveryCodes, Error> {
        let api_path = "/account/mfa/recovery-codes";

        let api_params = api_params! {};

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Regenerate MFA Recovery Codes
    ///
    /// Regenerate recovery codes that can be used as backup for MFA flow. Before
    /// regenerating codes, they must be first generated using
    /// [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes)
    /// method. An OTP challenge is required to regenerate recovery codes.
    pub async fn update_mfa_recovery_codes(client: &Client) -> Result<MfaRecoveryCodes, Error> {
        let api_path = "/account/mfa/recovery-codes";

        let api_params = api_params! {};

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PATCH, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update name
    ///
    /// Update currently logged in user account name.
    pub async fn update_name(client: &Client, name: &str) -> Result<User, Error> {
        const API_PATH: &str = "/account/name";

        let api_params = api_params! {
            "name"=> Some(name),
        };

        let api_headers = app_json_header!();

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
    pub async fn update_password(
        client: &Client,
        password: &str,
        old_password: &str,
    ) -> Result<User, Error> {
        const API_PATH: &str = "/account/name";

        let api_params = api_params! {
            "password"=> Some(password),
            "oldPassword"=> Some(old_password),
        };

        let api_headers = app_json_header!();

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
    pub async fn update_phone(client: &Client, phone: &str, password: &str) -> Result<User, Error> {
        const API_PATH: &str = "/account/name";

        let api_params = api_params! {
            "phone"=> Some(phone),
            "password"=>Some(password)
        };

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PATCH, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get account preferences
    ///
    /// Get the preferences as a key-value object for the currently logged in user.
    pub async fn get_preference(client: &Client) -> Result<Preferences, Error> {
        const API_PATH: &str = "/account/prefs";

        let api_params = api_params! {};

        let api_headers = app_json_header!();

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
    pub async fn update_preference(client: &Client, preference: Value) -> Result<User, Error> {
        const API_PATH: &str = "/account/prefs";

        let api_params = api_params! {
            "prefs"=> Some(preference),
        };

        let api_headers = app_json_header!();

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
    pub async fn create_recovery(client: &Client, email: &str, url: &str) -> Result<Token, Error> {
        const API_PATH: &str = "/account/recovery";

        let api_params = api_params! {
            "email"=> Some(email),
            "url"=> Some(url)
        };

        let api_headers = app_json_header!();

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
    pub async fn update_recovery(
        client: &Client,
        user_id: &str,
        secret: &str,
        password: &str,
        password_again: &str,
    ) -> Result<Token, Error> {
        const API_PATH: &str = "/account/recovery";

        let api_params = api_params!(
            "userId"=>Some(user_id),
            "secret"=> Some(secret),
            "password"=>Some(password),
            "passwordAgain"=>Some(password_again),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List sessions
    ///
    /// Get the list of active sessions across different devices for the currently
    /// logged in user.
    pub async fn list_sessions(client: &Client) -> Result<SessionList, Error> {
        const API_PATH: &str = "/account/sessions";

        let api_params = api_params!();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete sessions
    ///
    /// Delete all sessions from the user account and remove any sessions cookies
    /// from the end client.
    pub async fn delete_sessions(client: &Client) -> Result<(), Error> {
        const API_PATH: &str = "/account/sessions";

        let api_params = api_params!();

        let api_headers = app_json_header!();

        let _res = client
            .call(HttpMethod::DELETE, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(())
    }

    /// Create anonymous session
    ///
    /// Use this endpoint to allow a new user to register an anonymous account in
    /// your project. This route will also create a new session for the user. To
    /// allow the new user to convert an anonymous account to a normal account, you
    /// need to update its [email and
    /// password](https://appwrite.io/docs/references/cloud/client-web/account#updateEmail)
    /// or create an [OAuth2
    /// session](https://appwrite.io/docs/references/cloud/client-web/account#CreateOAuth2Session).
    pub async fn create_anonymous_session(client: &Client) -> Result<Session, Error> {
        let api_path = "/account/sessions/anonymous";

        let api_params = api_params!();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create email password session
    ///
    /// Allow the user to login into their account by providing a valid email and
    /// password combination. This route will create a new session for the user.
    ///
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    pub async fn create_email_password_session(
        client: &Client,
        email: &str,
        password: &str,
    ) -> Result<Session, Error> {
        let api_path = "/account/sessions/email";

        let api_params = api_params!(
            "email"=>Some(email),
            "password"=>Some(password),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update magic URL session
    ///
    /// Use this endpoint to create a session from token. Provide the **userId**
    /// and **secret** parameters from the successful response of authentication
    /// flows initiated by token creation. For example, magic URL and phone login.
    pub async fn create_magic_url_session(
        client: &Client,
        user_id: &str,
        secret: &str,
    ) -> Result<Session, Error> {
        let api_path = "/account/sessions/magic-url";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "secret"=> Some(secret),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update phone session
    ///
    /// Use this endpoint to create a session from token. Provide the **userId**
    /// and **secret** parameters from the successful response of authentication
    /// flows initiated by token creation. For example, magic URL and phone login.
    pub async fn update_phone_session(
        client: &Client,
        user_id: &str,
        secret: &str,
    ) -> Result<Session, Error> {
        let api_path = "/account/sessions/phone";

        let api_params = api_params!(
            "userId"=>Some(user_id),
            "secret"=>Some(secret),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create session
    ///
    /// Use this endpoint to create a session from token. Provide the **userId**
    /// and **secret** parameters from the successful response of authentication
    /// flows initiated by token creation. For example, magic URL and phone login.
    pub async fn create_session(
        client: &Client,
        user_id: &str,
        secret: &str,
    ) -> Result<Session, Error> {
        let api_path = "/account/sessions/token";

        let api_params = api_params!(
            "userId"=>Some(user_id),
            "secret"=>Some(secret),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get session
    ///
    /// Use this endpoint to get a logged in user's session using a Session ID.
    /// Inputting 'current' will return the current session being used.
    pub async fn get_session(client: &Client, session_id: &str) -> Result<Session, Error> {
        let api_path: String = r"/account/sessions/{sessionId}"
            .to_owned()
            .replace("{sessionId}", session_id);

        let api_params = api_params!();

        let api_headers = app_json_header!();

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

    /// Update session
    ///
    /// Use this endpoint to extend a session's length. Extending a session is
    /// useful when session expiry is short. If the session was created using an
    /// OAuth provider, this endpoint refreshes the access token from the provider.
    pub async fn update_session(client: &Client, session_id: &str) -> Result<Session, Error> {
        let api_path: String = r"/account/sessions/{sessionId}"
            .to_owned()
            .replace("{sessionId}", session_id);

        let api_params = api_params!();

        let api_headers = app_json_header!();

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
    pub async fn delete_session(client: &Client, session_id: &str) -> Result<(), Error> {
        let api_path: String = r"/account/sessions/{sessionId}"
            .to_owned()
            .replace("{sessionId}", session_id);

        let api_params = api_params!();

        let api_headers = app_json_header!();

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

        let api_params = api_params!();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::DELETE, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create email token (OTP)
    ///
    /// Sends the user an email with a secret key for creating a session. If the
    /// provided user ID has not be registered, a new user will be created. Use the
    /// returned user ID and secret and submit a request to the [POST
    /// /v1/account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession)
    /// endpoint to complete the login process. The secret sent to the user's email
    /// is valid for 15 minutes.
    ///
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    pub async fn create_email_token(
        client: &Client,
        user_id: &str,
        email: &str,
        phrase: Option<bool>,
    ) -> Result<Token, Error> {
        let api_path = "/account/token/email";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> Some(email),
            "phrase"=> phrase,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create magic URL token
    ///
    /// Sends the user an email with a secret key for creating a session. If the
    /// provided user ID has not been registered, a new user will be created. When
    /// the user clicks the link in the email, the user is redirected back to the
    /// URL you provided with the secret key and userId values attached to the URL
    /// query string. Use the query string parameters to submit a request to the
    /// [POST
    /// /v1/account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession)
    /// endpoint to complete the login process. The link sent to the user's email
    /// address is valid for 1 hour. If you are on a mobile device you can leave
    /// the URL parameter empty, so that the login completion will be handled by
    /// your Appwrite instance by default.
    ///
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    ///
    pub async fn create_magic_url_token(
        client: &Client,
        user_id: &str,
        email: &str,
        url: Option<&str>,
        phrase: Option<bool>,
    ) -> Result<Token, Error> {
        let api_path = "/account/token/magic-url";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> Some(email),
            "phrase"=> phrase,
            "url"=>url,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create OAuth2 token
    ///
    /// Allow the user to login to their account using the OAuth2 provider of their
    /// choice. Each OAuth2 provider should be enabled from the Appwrite console
    /// first. Use the success and failure arguments to provide a redirect URL's
    /// back to your app when login is completed.
    ///
    /// If authentication succeeds, `userId` and `secret` of a token will be
    /// appended to the success URL as query parameters. These can be used to
    /// create a new session using the [Create
    /// session](https://appwrite.io/docs/references/cloud/client-web/account#createSession)
    /// endpoint.
    ///
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    pub async fn create_oauth2_token(
        client: &Client,
        provider: OAuthProvider,
        success: Option<&str>,
        failure: Option<&str>,
        scopes: Option<Vec<&str>>,
    ) -> Result<(), Error> {
        let api_path = format!("/account/token/oauth2/{}", json!(provider));

        let api_params = api_params!(
            "success"=> success,
            "failure"=> failure,
            "scopes"=> scopes,
            "project"=>get_content_header_value(&client, "project"),
        );

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &api_params,
                None,
            )
            .await?;
        //todo ! return client.webauth(url);
        Ok(())
    }

    // Create phone token
    ///
    /// Sends the user an SMS with a secret key for creating a session. If the
    /// provided user ID has not be registered, a new user will be created. Use the
    /// returned user ID and secret and submit a request to the [POST
    /// /v1/account/sessions/token](https://appwrite.io/docs/references/cloud/client-web/account#createSession)
    /// endpoint to complete the login process. The secret sent to the user's phone
    /// is valid for 15 minutes.
    ///
    /// A user is limited to 10 active sessions at a time by default. [Learn more
    /// about session
    /// limits](https://appwrite.io/docs/authentication-security#limits).
    pub async fn create_phone_token(
        client: &Client,
        user_id: &str,
        phone: &str,
    ) -> Result<Token, Error> {
        let api_path = "/account/token/phone";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "phone"=> Some(phone),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
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
    pub async fn create_verification(client: &Client, url: &str) -> Result<Token, Error> {
        const API_PATH: &str = "/account/verification";

        let api_params = api_params!(
            "url"=>Some(url)
        );

        let api_headers = app_json_header!();

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
    pub async fn update_verification(
        client: &Client,
        user_id: &str,
        secret: &str,
    ) -> Result<Token, Error> {
        const API_PATH: &str = "/account/verification";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "secret"=> Some(secret),
        );

        let api_headers = app_json_header!();

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
    pub async fn create_phone_verification(client: &Client) -> Result<Token, Error> {
        const API_PATH: &str = "/account/verification/phone";

        let api_params = api_params!();

        let api_headers = app_json_header!();

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
    pub async fn update_phone_verification(
        client: &Client,
        user_id: &str,
        secret: &str,
    ) -> Result<Token, Error> {
        const API_PATH: &str = "/account/verification/phone";

        let api_params = api_params!(
            "userId"=>Some(user_id),
            "secret"=>Some(secret),
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, API_PATH, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }
}
