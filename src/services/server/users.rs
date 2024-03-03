use reqwest::header;
use serde_json::{json, Map, Value};

use crate::{
    client::Client,
    enums::HttpMethod,
    error::Error,
    models::{
        identity_list::IdentityList, log_list::LogList, membership_list::MembershipList,
        preferences::Preferences, session_list::SessionList, user::User, user_list::UserList,
    },
};

/// The Users service allows you to manage your project users.
pub struct Users;

impl Users {
    /// List users
    ///
    /// Get a list of all the project"s users. You can use the query params to
    /// filter your results.
    pub async fn list(
        client: &Client,
        queries: Option<Vec<&str>>,
        search: Option<&str>,
    ) -> Result<UserList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users";

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        if let Some(search) = search {
            api_params.insert("search".to_string(), json!(search));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

        let mut api_params = serde_json::Map::new();
        api_params.insert("userId".to_string(), json!(user_id));
        if let Some(email) = email {
            api_params.insert("email".to_string(), json!(email));
        }
        if let Some(phone) = phone {
            api_params.insert("phone".to_string(), json!(phone));
        }
        if let Some(password) = password {
            api_params.insert("password".to_string(), json!(password));
        }
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

        let mut api_params = serde_json::Map::new();
        api_params.insert("userId".to_string(), json!(user_id));
        api_params.insert("email".to_string(), json!(email));
        api_params.insert("password".to_string(), json!(password));
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }

        let api_params = Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

        let mut api_params = serde_json::Map::new();
        api_params.insert("userId".to_string(), json!(user_id));
        api_params.insert("email".to_string(), json!(email));
        api_params.insert("password".to_string(), json!(password));
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }

        let api_params = Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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
        queries: Option<Vec<&str>>,
        search: Option<&str>,
    ) -> Result<IdentityList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/identities";

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        if let Some(search) = search {
            api_params.insert("search".to_string(), json!(search));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

        let mut api_params = serde_json::Map::new();
        api_params.insert("userId".to_string(), json!(user_id));
        api_params.insert("email".to_string(), json!(email));
        api_params.insert("password".to_string(), json!(password));
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }

        let api_params = Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

        let mut api_params = serde_json::Map::new();
        api_params.insert("userId".to_string(), json!(user_id));
        api_params.insert("email".to_string(), json!(email));
        api_params.insert("password".to_string(), json!(password));
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }

        let api_params = Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

        let mut api_params = serde_json::Map::new();
        api_params.insert("userId".to_string(), json!(user_id));
        api_params.insert("email".to_string(), json!(email));
        api_params.insert("password".to_string(), json!(password));
        api_params.insert("passwordSalt".to_string(), json!(password_salt));
        api_params.insert("passwordCpu".to_string(), json!(password_cpu));
        api_params.insert("passwordMemory".to_string(), json!(password_memory));
        api_params.insert("passwordParallel".to_string(), json!(password_parallel));
        api_params.insert("passwordLength".to_string(), json!(password_length));
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }

        let api_params = Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

        let mut api_params = serde_json::Map::new();
        api_params.insert("userId".to_string(), json!(user_id));
        api_params.insert("email".to_string(), json!(email));
        api_params.insert("password".to_string(), json!(password));
        api_params.insert("passwordSalt".to_string(), json!(password_salt));
        api_params.insert(
            "passwordSaltSeparator".to_string(),
            json!(password_salt_separator),
        );
        api_params.insert("passwordSignerKey".to_string(), json!(password_signer_key));
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }

        let api_params = Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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
        password_version: &str,
        name: Option<&str>,
    ) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/sha";

        let mut api_params = serde_json::Map::new();
        api_params.insert("userId".to_string(), json!(user_id));
        api_params.insert("email".to_string(), json!(email));
        api_params.insert("password".to_string(), json!(password));
        api_params.insert("passwordVersion".to_string(), json!(password_version));
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }

        let api_params = Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

    /// Update email
    ///
    /// Update the user email by its unique ID.
    pub async fn update_email(client: &Client, user_id: &str, email: &str) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/email".replace("{userId}", user_id);

        let api_params = serde_json::json!({
            "email":email,
        });

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

        let api_params = serde_json::json!({
            "labels":labels,
        });

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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
        queries: Option<Vec<&str>>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/logs".replace("{userId}", user_id);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = &queries {
            api_params.insert("queries".to_string(), json!(queries));
        }

        let api_params = Value::Object(api_params);

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

    /// List user memberships
    ///
    /// Get the user membership list by its unique ID.
    pub async fn membership_list(client: &Client, user_id: &str) -> Result<MembershipList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/memberships".replace("{userId}", user_id);

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

    /// Update name
    ///
    /// Update the user name by its unique ID.
    pub async fn update_name(client: &Client, user_id: &str, name: &str) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/name".replace("{userId}", user_id);

        let api_params = serde_json::json!({
            "name":name
        });

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

        let api_params = serde_json::json!({
            "password":password
        });

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

    /// Update phone
    ///
    /// Update the user phone by its unique ID.
    pub async fn update_phone(client: &Client, user_id: &str, number: &str) -> Result<User, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/phone".replace("{userId}", user_id);

        let api_params = serde_json::json!({
            "number":number
        });

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

    /// Get user preferences
    ///
    /// Get the user preferences by its unique ID.
    pub async fn get_prefs(client: &Client, user_id: &str) -> Result<Preferences, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/prefs".replace("{userId}", user_id);

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

        let api_params = serde_json::json!({
            "prefs":prefs
        });

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

    /// List user sessions
    ///
    /// Get the user sessions list by its unique ID.
    pub async fn list_sessions(client: &Client, user_id: &str) -> Result<SessionList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/sessions".replace("{userId}", user_id);

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

    /// Delete user sessions
    ///
    /// Delete all user's sessions by using the user's unique ID.
    pub async fn delete_sessions(client: &Client, user_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/users/{userId}/sessions".replace("{userId}", user_id);

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

        let api_params = serde_json::json!({
            "status":status
        });

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

        let api_params = serde_json::json!({
            "emailVerification":email_verification
        });

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

        let api_params = serde_json::json!({
            "phoneVerification":phone_verification
        });

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
}
