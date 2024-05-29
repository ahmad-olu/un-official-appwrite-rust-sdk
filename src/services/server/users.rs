//! # Users
//!
//! The Users service allows you to manage your project users.

use serde_json::{json, Map, Value};

use crate::{
    api_params, app_json_header,
    client::Client,
    enumm::HttpMethod,
    enums::{
        authentication_type::AuthenticationType, messaging_provider_type::MessagingProviderType,
        password_hash::PasswordHash,
    },
    error::Error,
    models::{
        identity_list::IdentityList, log_list::LogList, membership_list::MembershipList,
        mfa_factors::MfaFactors, mfa_recovery_codes::MfaRecoveryCodes, preferences::Preferences,
        session::Session, session_list::SessionList, target::Target, target_list::TargetList,
        token::Token, user::User, user_list::UserList,
    },
};

pub struct Users;

impl Users {
    /// List users
    ///
    /// Get a list of all the project"s users. You can use the query params to
    /// filter your results.
    pub async fn list(
        client: &Client,
        queries: Option<Vec<String>>,
        search: Option<String>,
    ) -> Result<UserList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users";

        let api_params = api_params!(
            "queries"=> queries,
            "search"=> search,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user
    ///
    /// Create a new user.
    pub async fn create(
        client: &Client,
        user_id: &str,
        email: Option<&str>,
        phone: Option<&str>,
        password: Option<&str>,
        name: Option<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> email,
            "phone"=> phone,
            "password"=> password,
            "name"=> name,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with Argon2 password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [Argon2](https://en.wikipedia.org/wiki/Argon2) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_argon2_user(
        client: &Client,
        user_id: &str,
        email: &str,
        password: &str,
        name: Option<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/argon2";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> Some(email),
            "password"=> Some(password),
            "name"=> name,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with bcrypt password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [Bcrypt](https://en.wikipedia.org/wiki/Bcrypt) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_bcrypt_user(
        client: &Client,
        user_id: &str,
        email: &str,
        password: &str,
        name: Option<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/bcrypt";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> Some(email),
            "password"=> Some(password),
            "name"=> name,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List Identities
    ///
    /// Get identities for all users.
    pub async fn list_identities(
        client: &Client,
        queries: Option<Vec<String>>,
        search: Option<String>,
    ) -> Result<IdentityList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/identities";

        let api_params = api_params!(
            "queries"=> queries,
            "search"=> search,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete Identity
    ///
    /// Delete an identity by its unique ID.
    pub async fn delete_identity(client: &Client, identity_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/identities/{identityId}".replace("{identityId}", identity_id);

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
            .await?;

        Ok(())
    }

    /// Create user with MD5 password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [MD5](https://en.wikipedia.org/wiki/MD5) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_md5_user(
        client: &Client,
        user_id: &str,
        email: &str,
        password: &str,
        name: Option<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/md5";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> Some(email),
            "password"=> Some(password),
            "name"=> name,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with PHPass password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [PHPass](https://www.openwall.com/phpass/) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_phpass_user(
        client: &Client,
        user_id: &str,
        email: &str,
        password: &str,
        name: Option<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/phpass";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> Some(email),
            "password"=> Some(password),
            "name"=> name,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with Scrypt password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [Scrypt](https://github.com/Tarsnap/scrypt) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_scrypt_user(
        client: &Client,
        user_id: &str,
        email: &str,
        password: &str,
        password_salt: &str,
        password_cpu: usize,
        password_memory: usize,
        password_parallel: usize,
        password_length: usize,
        name: Option<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/scrypt";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> Some(email),
            "password"=> Some(password),
            "passwordSalt"=> Some(password_salt),
            "passwordCpu"=> Some(password_cpu),
            "passwordMemory"=> Some(password_memory),
            "passwordParallel"=> Some(password_parallel),
            "passwordLength"=> Some(password_length),
            "name"=> name,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with Scrypt modified password
    ///
    /// Create a new user. Password provided must be hashed with the [Scrypt
    /// Modified](https://gist.github.com/Meldiron/eecf84a0225eccb5a378d45bb27462cc)
    /// algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    pub async fn create_scrypt_modified_user(
        client: &Client,
        user_id: &str,
        email: &str,
        password: &str,
        password_salt: &str,
        password_salt_separator: &str,
        password_signer_key: &str,
        name: Option<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/scrypt-modified";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> Some(email),
            "password"=> Some(password),
            "passwordSalt"=> Some(password_salt),
            "passwordSaltSeparator"=> Some(password_salt_separator),
            "passwordSignerKey"=> Some(password_signer_key),
            "name"=> name,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with SHA password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [SHA](https://en.wikipedia.org/wiki/Secure_Hash_Algorithm) algorithm. Use
    /// the [POST /users](https://appwrite.io/docs/server/users#usersCreate)
    /// endpoint to create users with a plain text password.
    pub async fn create_sha_user(
        client: &Client,
        user_id: &str,
        email: &str,
        password: &str,
        password_version: PasswordHash,
        name: Option<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/sha";

        let api_params = api_params!(
            "userId"=> Some(user_id),
            "email"=> Some(email),
            "password"=> Some(password),
            "passwordVersion"=> Some(password_version),
            "name"=> name,
        );

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get user
    ///
    /// Get a user by its unique ID.
    pub async fn get(client: &Client, user_id: &str) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}".replace("{userId}", user_id);

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

    /// Delete user
    ///
    /// Delete a user by its unique ID, thereby releasing it's ID. Since ID is
    /// released and can be reused, all user-related resources like documents or
    /// storage files should be deleted before user deletion. If you want to keep
    /// ID reserved, use the
    /// [updateStatus](https://appwrite.io/docs/server/users#usersUpdateStatus)
    /// endpoint instead.
    pub async fn delete(client: &Client, user_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}".replace("{userId}", user_id);

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
            .await?;

        Ok(())
    }

    /// Update email
    ///
    /// Update the user email by its unique ID.
    pub async fn update_email(client: &Client, user_id: &str, email: &str) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/email".replace("{userId}", user_id);

        let api_params = api_params!(
            "email"=> Some(email),
        );

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

    /// Update user labels
    ///
    /// Update the user labels by its unique ID.
    ///
    /// Labels can be used to grant access to resources. While teams are a way for
    /// user's to share access to a resource, labels can be defined by the
    /// developer to grant access without an invitation. See the [Permissions
    /// docs](https://appwrite.io/docs/permissions) for more info.
    pub async fn update_labels(
        client: &Client,
        user_id: &str,
        labels: Vec<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/labels".replace("{userId}", user_id);

        let api_params = api_params!(
            "labels"=> Some(labels),
        );

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

    /// List user logs
    ///
    /// Get the user activity logs list by its unique ID.
    pub async fn list_logs(
        client: &Client,
        user_id: &str,
        queries: Option<Vec<String>>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/logs".replace("{userId}", user_id);

        let api_params = api_params!(
            "queries"=> queries,
        );

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

    /// List user memberships
    ///
    /// Get the user membership list by its unique ID.
    pub async fn list_membership(client: &Client, user_id: &str) -> Result<MembershipList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/memberships".replace("{userId}", user_id);

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

    /// Update MFA
    ///
    /// Enable or disable MFA on a user account.
    pub async fn update_mfa(client: &Client, user_id: &str, mfa: bool) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/mfa".replace("{userId}", user_id);

        let api_params = api_params!(
            "mfa"=> Some(mfa),
        );

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

    /// Delete Authenticator
    ///
    /// Delete an authenticator app.
    pub async fn delete_mfa_authenticator(
        client: &Client,
        user_id: &str,
        auth_type: AuthenticationType,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/users/{}/mfa/authenticators/{}", user_id, json!(auth_type));

        let api_params = api_params!();

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

    /// List Factors
    ///
    /// List the factors available on the account to be used as a MFA challange.
    pub async fn list_mfa_factors(client: &Client, user_id: &str) -> Result<MfaFactors, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/users/{}/mfa/factors", user_id);

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

    /// Get MFA Recovery Codes
    ///
    /// Get recovery codes that can be used as backup for MFA flow by User ID.
    /// Before getting codes, they must be generated using
    /// [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes)
    /// method.
    pub async fn get_mfa_recovery_codes(
        client: &Client,
        user_id: &str,
    ) -> Result<MfaRecoveryCodes, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/users/{}/mfa/recovery-codes", user_id);

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

    /// Regenerate MFA Recovery Codes
    ///
    /// Regenerate recovery codes that can be used as backup for MFA flow by User
    /// ID. Before regenerating codes, they must be first generated using
    /// [createMfaRecoveryCodes](/docs/references/cloud/client-web/account#createMfaRecoveryCodes)
    /// method.
    pub async fn update_mfa_recovery_codes(
        client: &Client,
        user_id: &str,
    ) -> Result<MfaRecoveryCodes, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/users/{}/mfa/recovery-codes", user_id);

        let api_params = api_params!();

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

    /// Create MFA Recovery Codes
    ///
    /// Generate recovery codes used as backup for MFA flow for User ID. Recovery
    /// codes can be used as a MFA verification type in
    /// [createMfaChallenge](/docs/references/cloud/client-web/account#createMfaChallenge)
    /// method by client SDK.
    pub async fn create_mfa_recovery_codes(
        client: &Client,
        user_id: &str,
    ) -> Result<MfaRecoveryCodes, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/users/{}/mfa/recovery-codes", user_id);

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

    /// Update name
    ///
    /// Update the user name by its unique ID.
    pub async fn update_name(client: &Client, user_id: &str, name: &str) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/name".replace("{userId}", user_id);

        let api_params = api_params!(
            "name"=> Some(name),
        );

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

    /// Update password
    ///
    /// Update the user password by its unique ID.
    pub async fn update_password(
        client: &Client,
        user_id: &str,
        password: &str,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/password".replace("{userId}", user_id);

        let api_params = api_params!(
            "password"=> Some(password),
        );

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

    /// Update phone
    ///
    /// Update the user phone by its unique ID.
    pub async fn update_phone(client: &Client, user_id: &str, number: &str) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/phone".replace("{userId}", user_id);

        let api_params = api_params!(
            "number"=> Some(number),
        );

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

    /// Get user preferences
    ///
    /// Get the user preferences by its unique ID.
    pub async fn get_prefs(client: &Client, user_id: &str) -> Result<Preferences, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/prefs".replace("{userId}", user_id);

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

    /// Update user preferences
    ///
    /// Update the user preferences by its unique ID. The object you pass is stored
    /// as is, and replaces any previous value. The maximum allowed prefs size is
    /// 64kB and throws error if exceeded.
    pub async fn update_prefs(
        client: &Client,
        user_id: &str,
        prefs: Map<String, Value>,
    ) -> Result<Preferences, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/prefs".replace("{userId}", user_id);

        let api_params = api_params!(
            "prefs"=> Some(prefs),
        );

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

    /// List user sessions
    ///
    /// Get the user sessions list by its unique ID.
    pub async fn list_sessions(client: &Client, user_id: &str) -> Result<SessionList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/sessions".replace("{userId}", user_id);

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

    /// Create session
    ///
    /// Creates a session for a user. Returns an immediately usable session object.
    ///
    /// If you want to generate a token for a custom authentication flow, use the
    /// [POST
    /// /users/{userId}/tokens](https://appwrite.io/docs/server/users#createToken)
    /// endpoint.
    pub async fn create_sessions(client: &Client, user_id: &str) -> Result<Session, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/sessions".replace("{userId}", user_id);

        let api_params = api_params!();

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

    /// Delete user sessions
    ///
    /// Delete all user's sessions by using the user's unique ID.
    pub async fn delete_sessions(client: &Client, user_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/sessions".replace("{userId}", user_id);

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
            .await?;

        Ok(())
    }

    /// Delete user session
    ///
    /// Delete a user sessions by its unique ID.
    pub async fn delete_session(
        client: &Client,
        user_id: &str,
        session_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/sessions/{sessionId}"
            .replace("{userId}", user_id)
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
            .await?;

        Ok(())
    }

    /// Update user status
    ///
    /// Update the user status by its unique ID. Use this endpoint as an
    /// alternative to deleting a user if you want to keep user's ID reserved.
    pub async fn update_status(
        client: &Client,
        user_id: &str,
        status: bool,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/status".replace("{userId}", user_id);

        let api_params = api_params!(
            "status"=> Some(status),
        );

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

    /// List User Targets
    ///
    /// List the messaging targets that are associated with a user.
    pub async fn list_targets(
        client: &Client,
        user_id: &str,
        queries: Option<Vec<String>>,
    ) -> Result<TargetList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/targets".replace("{userId}", user_id);

        let api_params = api_params!(
            "queries"=> queries,
        );

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

    /// Create User Target
    ///
    /// Create a messaging target.
    pub async fn create_target(
        client: &Client,
        user_id: &str,
        target_id: &str,
        provider_type: MessagingProviderType,
        identifier: &str,
        provider_id: Option<&str>,
        name: Option<&str>,
    ) -> Result<Target, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/targets".replace("{userId}", user_id);

        let api_params = api_params!(
            "targetId"=> Some(target_id),
            "providerType"=> Some(provider_type),
            "identifier"=> Some(identifier),
            "providerId"=>provider_id,
            "name"=> name,
        );

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

    /// Get User Target
    ///
    /// Get a user's push notification target by ID.
    pub async fn get_target(
        client: &Client,
        user_id: &str,
        target_id: &str,
    ) -> Result<Target, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/targets/{targetId}"
            .replace("{userId}", user_id)
            .replace("{targetId}", target_id);

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

    /// Update User target
    ///
    /// Update a messaging target.
    pub async fn update_target(
        client: &Client,
        user_id: &str,
        target_id: &str,
        identifier: Option<&str>,
        provider_id: Option<&str>,
        name: Option<&str>,
    ) -> Result<Target, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/targets/{targetId}"
            .replace("{userId}", user_id)
            .replace("{targetId}", target_id);

        let api_params = api_params!(
            "identifier"=> Some(identifier),
            "providerId"=> Some(provider_id),
            "name"=> Some(name),
        );

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

    /// Delete user target
    ///
    /// Delete a messaging target.
    pub async fn delete_target(
        client: &Client,
        user_id: &str,
        target_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/targets/{targetId}"
            .replace("{userId}", user_id)
            .replace("{targetId}", target_id);

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
            .await?;

        Ok(())
    }

    /// Create token
    ///
    /// Returns a token with a secret key for creating a session. If the provided
    /// user ID has not be registered, a new user will be created. Use the returned
    /// user ID and secret and submit a request to the [PUT
    /// /account/sessions/custom](https://appwrite.io/docs/references/cloud/client-web/account#updateCustomSession)
    /// endpoint to complete the login process.
    pub async fn create_token(
        client: &Client,
        user_id: &str,
        length: Option<usize>,
        expire: Option<usize>,
    ) -> Result<Token, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/tokens".replace("{userId}", user_id);

        let api_params = api_params!(
            "length"=> length,
            "expire"=>expire,
        );

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

    /// Update email verification
    ///
    /// Update the user email verification status by its unique ID.
    pub async fn update_email_verification(
        client: &Client,
        user_id: &str,
        email_verification: bool,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/verification".replace("{userId}", user_id);

        let api_params = api_params!(
            "emailVerification"=> Some(email_verification),
        );

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

    /// Update phone verification
    ///
    /// Update the user phone verification status by its unique ID.
    pub async fn update_phone_verification(
        client: &Client,
        user_id: &str,
        phone_verification: bool,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/verification/phone".replace("{userId}", user_id);

        let api_params = api_params!(
            "phoneVerification"=> Some(phone_verification),
        );

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
}
