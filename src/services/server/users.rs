// //! # Users
// //!
// //! The Users service allows you to manage your project users.

use std::collections::HashMap;

use serde_json::{json, Value};

use crate::{
    app_json_header,
    client::Client,
    enumm::HttpMethod,
    enums::authentication_type::AuthenticationType,
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
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list(client: &Client, args: HashMap<String, Value>) -> Result<UserList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user
    ///
    /// Create a new user.
    ///
    ///* userId => string
    ///* email => string?
    ///* phone => string?
    ///* password => string?
    ///* name => string?
    pub async fn create(client: &Client, args: HashMap<String, Value>) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with Argon2 password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [Argon2](https://en.wikipedia.org/wiki/Argon2) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    ///* userId => string
    ///* email => string
    ///* password => string
    ///* name => string?
    pub async fn create_argon2_user(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/argon2";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with bcrypt password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [Bcrypt](https://en.wikipedia.org/wiki/Bcrypt) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    ///* userId => string
    ///* email => string
    ///* password => string
    ///* name => string?
    pub async fn create_bcrypt_user(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/bcrypt";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List Identities
    ///
    /// Get identities for all users.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list_identities(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<IdentityList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/identities";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete Identity
    ///
    /// Delete an identity by its unique ID.
    pub async fn delete_identity(client: &Client, identity_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/identities/{identityId}".replace("{identityId}", identity_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                args,
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
    ///* userId => string
    ///* email => string
    ///* password => string
    ///* name => string?
    pub async fn create_md5_user(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/md5";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with PHPass password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [PHPass](https://www.openwall.com/phpass/) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    ///* userId => string
    ///* email => string
    ///* password => string
    ///* name => string?
    pub async fn create_phpass_user(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/phpass";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with Scrypt password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [Scrypt](https://github.com/Tarsnap/scrypt) algorithm. Use the [POST
    /// /users](https://appwrite.io/docs/server/users#usersCreate) endpoint to
    /// create users with a plain text password.
    ///* userId => string
    ///* email => string
    ///* password => string
    ///* passwordSalt => string
    ///* passwordCpu => usize / u16 / number
    ///* passwordMemory => usize / u16 / number
    ///* passwordParallel => usize / u16 / number
    ///* passwordLength => usize / u16 / number
    ///* name => string?
    pub async fn create_scrypt_user(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/scrypt";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
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
    ///* userId => string
    ///* email => string
    ///* password => string
    ///* passwordSalt => string
    ///* passwordSaltSeparator => string
    ///* passwordSignerKey => string
    ///* name => string?
    pub async fn create_scrypt_modified_user(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/scrypt-modified";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create user with SHA password
    ///
    /// Create a new user. Password provided must be hashed with the
    /// [SHA](https://en.wikipedia.org/wiki/Secure_Hash_Algorithm) algorithm. Use
    /// the [POST /users](https://appwrite.io/docs/server/users#usersCreate)
    /// endpoint to create users with a plain text password.
    ///* userId => string
    ///* email => string
    ///* password => string
    ///* passwordVersion => PasswordHash
    ///* name => string?
    pub async fn create_sha_user(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/sha";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get user
    ///
    /// Get a user by its unique ID.
    pub async fn get(client: &Client, user_id: &str) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}".replace("{userId}", user_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                args,
                None,
            )
            .await?;

        Ok(())
    }

    /// Update email
    ///
    /// Update the user email by its unique ID.
    ///* email => string
    pub async fn update_email(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/email".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Create user JWT
    ///
    /// Use this endpoint to create a JSON Web Token for user by its unique ID. You
    /// can use the resulting JWT to authenticate on behalf of the user. The JWT
    /// secret will become invalid if the session it uses gets deleted.
    ///* sessionId => string?
    ///* duration => number?
    pub async fn update_jwt(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/jwts".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
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
    ///* labels => vec(string)
    pub async fn update_labels(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/labels".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List user logs
    ///
    /// Get the user activity logs list by its unique ID.
    ///* queries => vec(string)?
    pub async fn list_logs(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/logs".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List user memberships
    ///
    /// Get the user membership list by its unique ID.
    pub async fn list_membership(client: &Client, user_id: &str) -> Result<MembershipList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/memberships".replace("{userId}", user_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update MFA
    ///
    /// Enable or disable MFA on a user account.
    ///* mfa => bool
    pub async fn update_mfa(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/mfa".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                args,
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, args, None)
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update name
    ///
    /// Update the user name by its unique ID.
    ///* name => string
    pub async fn update_name(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/name".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update password
    ///
    /// Update the user password by its unique ID.
    ///* password => string
    pub async fn update_password(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/password".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update phone
    ///
    /// Update the user phone by its unique ID.
    ///* number => string
    pub async fn update_phone(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/phone".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update user preferences
    ///
    /// Update the user preferences by its unique ID. The object you pass is stored
    /// as is, and replaces any previous value. The maximum allowed prefs size is
    /// 64kB and throws error if exceeded.
    ///* prefs => HashMap<String, Value>
    pub async fn update_prefs(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Preferences, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/prefs".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete user sessions
    ///
    /// Delete all user's sessions by using the user's unique ID.
    pub async fn delete_sessions(client: &Client, user_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/sessions".replace("{userId}", user_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                args,
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                args,
                None,
            )
            .await?;

        Ok(())
    }

    /// Update user status
    ///
    /// Update the user status by its unique ID. Use this endpoint as an
    /// alternative to deleting a user if you want to keep user's ID reserved.
    ///* status => bool
    pub async fn update_status(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/status".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// List User Targets
    ///
    /// List the messaging targets that are associated with a user.
    ///* queries => vec(string)?
    pub async fn list_targets(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<TargetList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/targets".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create User Target
    ///
    /// Create a messaging target.
    ///* targetId => string
    ///* providerType => MessagingProviderType
    ///* identifier => string
    ///* providerId => string?
    ///* name => string?
    pub async fn create_target(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Target, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/targets".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path.as_str(), api_headers, args, None)
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update User target
    ///
    /// Update a messaging target.
    ///* identifier => string?
    ///* providerId => string?
    ///* name => string?
    pub async fn update_target(
        client: &Client,
        user_id: &str,
        target_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Target, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/targets/{targetId}"
            .replace("{userId}", user_id)
            .replace("{targetId}", target_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                args,
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
    ///* length => number?
    ///* expire => number?
    pub async fn create_token(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Token, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/tokens".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update email verification
    ///
    /// Update the user email verification status by its unique ID.
    ///* emailVerification => bool
    pub async fn update_email_verification(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/verification".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Update phone verification
    ///
    /// Update the user phone verification status by its unique ID.
    ///* phoneVerification => bool
    pub async fn update_phone_verification(
        client: &Client,
        user_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/verification/phone".replace("{userId}", user_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{Map, Value};

    use crate::{client::ClientBuilder, enums::password_hash::PasswordHash, error::Error, id::ID};

    use super::Users;

    #[tokio::test]
    async fn test_user() -> Result<(), Error> {
        let client = ClientBuilder::default()
            .set_endpoint("http://127.0.0.1/v1")?
            .set_project("676c2b7b000c834e1fce")?
            .set_key("standard_5d84014ebaf0de52308eff28946a43062921240c10b81c2fd037ab60b02f0257b7f0a53fe94065170fe7c7d0af2d4136d4cbf32a4055baeada3d27f2e323b70aeda87e97f676207cf10cbb18b7a80f8d1103803617454c89138f217dad701bbe9dc6950bc58853fdb2a0b4b67d2a8b8b6b7b9b2e6d9b94e0a2fcfee794688e2e")?
            //.set_self_signed(false)?
            .build()?;

        // ! create user
        let user_res = Users::create(
            &client,
            maplit::hashmap! {
                "userId".into() => ID::unique(7).into(),
                "email".into()=> "fakeEmail@Email.com".into(),
                "password".into()=> "VeryVerySecurePassword@123456789".into(),
                "name".into()=> "fakeEmail".into()
            },
        )
        .await?;
        assert_eq!(user_res.email, "fakeemail@email.com");

        // ! update email
        let update_email = Users::update_email(
            &client,
            &user_res.id,
            maplit::hashmap! {
                "email".into()=> "newEmail@Email.com".into(),
            },
        )
        .await?;
        assert_eq!(update_email.email, "newemail@email.com");

        // ! update name
        let update_name = Users::update_name(
            &client,
            &user_res.id,
            maplit::hashmap! {
                "name".into()=> "Micky".into(),
            },
        )
        .await?;
        assert_eq!(update_name.name, "Micky");

        // ! update password
        let update_password = Users::update_password(
            &client,
            &user_res.id,
            maplit::hashmap! {
                "password".into()=> "ChangedMyVerySecuredPassword".into(),
            },
        )
        .await?;
        assert_ne!(user_res.password_update, update_password.password_update);
        assert_ne!(user_res.password, update_password.password);

        // ! update preferences
        let prefs_val: HashMap<String, Value> = maplit::hashmap! {
            "is_king".into()=> true.into(),
        };
        // ? NOTE => because `serde_json::Value` does not directly implement `From<HashMap<String, Value>>`. You need to convert the `HashMap<String, Value>` into a `serde_json::Value` explicitly using `serde_json::Value::Object`.
        let update_prefs = Users::update_prefs(
            &client,
            &user_res.id,
            maplit::hashmap! {
                "prefs".into()=> Value::Object(prefs_val.into_iter().collect::<Map<String, Value>>()),
            },
        )
        .await?;
        let ans = maplit::hashmap! {
            "is_king".into()=> true.into(),
        };
        assert_eq!(update_prefs.data, ans);

        // ! get preferences
        let get_prefs = Users::get_prefs(&client, &user_res.id).await?;
        let ans = maplit::hashmap! {
            "is_king".into()=> true.into(),
        };
        assert_eq!(get_prefs.data, ans);

        // ! delete user
        let delete_user = Users::delete(&client, &user_res.id).await?;
        assert_eq!(delete_user, ());

        // ! create argon2 user
        let argon2_user_res = Users::create_argon2_user(
            &client,
            maplit::hashmap! {
                "userId".into() => ID::unique(7).into(),
                "email".into()=> "argonEmail@Email.com".into(),
                "password".into()=> "VeryVerySecurePassword@123456789".into(),
            },
        )
        .await?;
        assert_eq!(argon2_user_res.email, "argonemail@email.com");

        // ! create sha user
        let sha_user_res = Users::create_sha_user(
            &client,
            maplit::hashmap! {
                "userId".into() => ID::unique(7).into(),
                "email".into()=> "shaEmail@Email.com".into(),
                "password".into()=> "VeryVerySecurePassword@123456789".into(),
                "passwordVersion".into()=> PasswordHash::Sha3384.as_serialized().into(),
            },
        )
        .await?;
        assert_eq!(sha_user_res.email, "shaemail@email.com");

        let delete_argon_user = Users::delete(&client, &argon2_user_res.id).await?;
        assert_eq!(delete_argon_user, ());

        let delete_sha_user = Users::delete(&client, &sha_user_res.id).await?;
        assert_eq!(delete_sha_user, ());

        Ok(())
    }
}
