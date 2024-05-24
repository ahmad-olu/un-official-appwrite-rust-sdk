//! # Messaging
//!
//! The Messaging service allows you to send messages to any provider type
//! (SMTP, push notification, SMS, etc.).

use reqwest::header;
use serde_json::{json, Map, Value};

use crate::{
    client::Client,
    enumm::HttpMethod,
    enums::smtp_encryption::SmtpEncryption,
    error::Error,
    models::{
        log_list::LogList, message::Message, message_list::MessageList, provider::Provider,
        provider_list::ProviderList, subscriber::Subscriber, subscriber_list::SubscriberList,
        target_list::TargetList, topic::Topic, topic_list::TopicList,
    },
};

pub struct Messaging;

impl Messaging {
    /// List messages
    ///
    /// Get a list of all messages from the current Appwrite project.
    pub async fn list_messages(
        client: &Client,
        queries: Option<Vec<String>>,
        search: Option<String>,
    ) -> Result<MessageList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/messages";

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

    /// Create email
    ///
    /// Create a new email message.
    pub async fn create_email(
        client: &Client,
        message_id: &str,
        subject: &str,
        content: &str,
        topics: Option<Vec<&str>>,
        users: Option<Vec<&str>>,
        targets: Option<Vec<&str>>,
        cc: Option<Vec<&str>>,
        bcc: Option<Vec<&str>>,
        attachments: Option<Vec<&str>>,
        draft: Option<bool>,
        html: Option<bool>,
        scheduled_at: Option<bool>,
    ) -> Result<Message, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/messages/email";

        let mut api_params = serde_json::Map::new();
        api_params.insert("messageId".to_string(), json!(message_id));
        api_params.insert("subject".to_string(), json!(subject));
        api_params.insert("content".to_string(), json!(content));

        if let Some(topics) = topics {
            api_params.insert("topics".to_string(), json!(topics));
        }
        if let Some(users) = users {
            api_params.insert("users".to_string(), json!(users));
        }
        if let Some(targets) = targets {
            api_params.insert("targets".to_string(), json!(targets));
        }
        if let Some(cc) = cc {
            api_params.insert("cc".to_string(), json!(cc));
        }
        if let Some(bcc) = bcc {
            api_params.insert("bcc".to_string(), json!(bcc));
        }
        if let Some(attachments) = attachments {
            api_params.insert("attachments".to_string(), json!(attachments));
        }
        if let Some(draft) = draft {
            api_params.insert("draft".to_string(), json!(draft));
        }
        if let Some(html) = html {
            api_params.insert("html".to_string(), json!(html));
        }
        if let Some(scheduled_at) = scheduled_at {
            api_params.insert("scheduleAt".to_string(), json!(scheduled_at));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update email
    ///
    /// Update an email message by its unique ID.
    ///
    pub async fn update_email(
        client: &Client,
        message_id: &str,
        topics: Option<Vec<&str>>,
        users: Option<Vec<&str>>,
        targets: Option<Vec<&str>>,
        subject: Option<&str>,
        content: Option<&str>,
        draft: Option<bool>,
        html: Option<bool>,
        cc: Option<Vec<&str>>,
        bcc: Option<Vec<&str>>,
        scheduled_at: Option<&str>,
    ) -> Result<Message, Error> {
        //let api_path = "/messaging/messages/email";
        let api_path = "/messaging/messages/email/{messageId}".replace("{messageId}", message_id);

        let mut api_params = serde_json::Map::new();
        if let Some(topics) = topics {
            api_params.insert("topics".to_string(), json!(topics));
        }
        if let Some(users) = users {
            api_params.insert("users".to_string(), json!(users));
        }
        if let Some(targets) = targets {
            api_params.insert("targets".to_string(), json!(targets));
        }
        if let Some(subject) = subject {
            api_params.insert("subject".to_string(), json!(subject));
        }
        if let Some(content) = content {
            api_params.insert("content".to_string(), json!(content));
        }
        if let Some(draft) = draft {
            api_params.insert("draft".to_string(), json!(draft));
        }
        if let Some(html) = html {
            api_params.insert("html".to_string(), json!(html));
        }
        if let Some(cc) = cc {
            api_params.insert("cc".to_string(), json!(cc));
        }
        if let Some(bcc) = bcc {
            api_params.insert("bcc".to_string(), json!(bcc));
        }
        if let Some(scheduled_at) = scheduled_at {
            api_params.insert("scheduledAt".to_string(), json!(scheduled_at));
        }

        let api_params = serde_json::Value::Object(api_params);

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

    /// Create push notification
    ///
    /// Create a new push notification.
    pub async fn create_push(
        client: &Client,
        message_id: &str,
        title: &str,
        body: &str,
        topics: Option<Vec<&str>>,
        users: Option<Vec<&str>>,
        targets: Option<Vec<&str>>,
        data: Option<Map<String, Value>>,
        action: Option<&str>,
        image: Option<&str>,
        icon: Option<&str>,
        sound: Option<&str>,
        color: Option<&str>,
        tag: Option<&str>,
        badge: Option<&str>,
        draft: Option<bool>,
        scheduled_at: Option<&str>,
    ) -> Result<Message, Error> {
        let api_path = "/messaging/messages/push";

        let mut api_params = serde_json::Map::new();
        api_params.insert("messageId".to_string(), json!(message_id));
        api_params.insert("title".to_string(), json!(title));
        api_params.insert("body".to_string(), json!(body));
        if let Some(topics) = topics {
            api_params.insert("topics".to_string(), json!(topics));
        }
        if let Some(users) = users {
            api_params.insert("users".to_string(), json!(users));
        }
        if let Some(targets) = targets {
            api_params.insert("targets".to_string(), json!(targets));
        }
        if let Some(data) = data {
            api_params.insert("data".to_string(), json!(data));
        }
        if let Some(action) = action {
            api_params.insert("action".to_string(), json!(action));
        }
        if let Some(image) = image {
            api_params.insert("image".to_string(), json!(image));
        }
        if let Some(icon) = icon {
            api_params.insert("icon".to_string(), json!(icon));
        }
        if let Some(sound) = sound {
            api_params.insert("sound".to_string(), json!(sound));
        }
        if let Some(color) = color {
            api_params.insert("color".to_string(), json!(color));
        }
        if let Some(tag) = tag {
            api_params.insert("tag".to_string(), json!(tag));
        }
        if let Some(badge) = badge {
            api_params.insert("badge".to_string(), json!(badge));
        }
        if let Some(draft) = draft {
            api_params.insert("draft".to_string(), json!(draft));
        }
        if let Some(scheduled_at) = scheduled_at {
            api_params.insert("scheduledAt".to_string(), json!(scheduled_at));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update push notification
    ///
    /// Update a push notification by its unique ID.
    ///
    pub async fn update_push(
        client: &Client,
        message_id: &str,
        topics: Option<Vec<&str>>,
        users: Option<Vec<&str>>,
        targets: Option<Vec<&str>>,
        title: Option<&str>,
        body: Option<&str>,
        data: Option<Map<String, Value>>,
        action: Option<&str>,
        image: Option<&str>,
        icon: Option<&str>,
        sound: Option<&str>,
        color: Option<&str>,
        tag: Option<&str>,
        badge: Option<i32>,
        draft: Option<bool>,
        scheduled_at: Option<&str>,
    ) -> Result<Message, Error> {
        let api_path = format!("/messaging/messages/push/{}", message_id);

        let mut api_params = serde_json::Map::new();
        if let Some(topics) = topics {
            api_params.insert("topics".to_string(), json!(topics));
        }
        if let Some(users) = users {
            api_params.insert("users".to_string(), json!(users));
        }
        if let Some(targets) = targets {
            api_params.insert("targets".to_string(), json!(targets));
        }
        if let Some(title) = title {
            api_params.insert("title".to_string(), json!(title));
        }
        if let Some(body) = body {
            api_params.insert("body".to_string(), json!(body));
        }
        if let Some(data) = data {
            api_params.insert("data".to_string(), json!(data));
        }
        if let Some(action) = action {
            api_params.insert("action".to_string(), json!(action));
        }
        if let Some(image) = image {
            api_params.insert("image".to_string(), json!(image));
        }
        if let Some(icon) = icon {
            api_params.insert("icon".to_string(), json!(icon));
        }
        if let Some(sound) = sound {
            api_params.insert("sound".to_string(), json!(sound));
        }
        if let Some(color) = color {
            api_params.insert("color".to_string(), json!(color));
        }
        if let Some(tag) = tag {
            api_params.insert("tag".to_string(), json!(tag));
        }
        if let Some(badge) = badge {
            api_params.insert("badge".to_string(), json!(badge));
        }
        if let Some(draft) = draft {
            api_params.insert("draft".to_string(), json!(draft));
        }
        if let Some(scheduled_at) = scheduled_at {
            api_params.insert("scheduledAt".to_string(), json!(scheduled_at));
        }

        let api_params = serde_json::Value::Object(api_params);

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

    /// Create SMS
    ///
    /// Create a new SMS message.
    pub async fn create_sms(
        client: &Client,
        message_id: &str,
        content: &str,
        topics: Option<Vec<&str>>,
        users: Option<Vec<&str>>,
        targets: Option<Vec<&str>>,
        draft: Option<bool>,
        scheduled_at: Option<&str>,
    ) -> Result<Message, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/messages/sms";

        let mut api_params = serde_json::Map::new();
        api_params.insert("messageId".to_string(), json!(message_id));
        api_params.insert("content".to_string(), json!(content));
        if let Some(topics) = topics {
            api_params.insert("topics".to_string(), json!(topics));
        }
        if let Some(users) = users {
            api_params.insert("users".to_string(), json!(users));
        }
        if let Some(targets) = targets {
            api_params.insert("targets".to_string(), json!(targets));
        }
        if let Some(draft) = draft {
            api_params.insert("draft".to_string(), json!(draft));
        }
        if let Some(scheduled_at) = scheduled_at {
            api_params.insert("scheduledAt".to_string(), json!(scheduled_at));
        }

        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update SMS
    ///
    /// Update an email message by its unique ID.
    ///
    pub async fn update_sms(
        client: &Client,
        message_id: &str,
        topics: Option<Vec<&str>>,
        users: Option<Vec<&str>>,
        targets: Option<Vec<&str>>,
        content: Option<&str>,
        draft: Option<bool>,
        scheduled_at: Option<&str>,
    ) -> Result<Message, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/messages/sms/{}", message_id);

        let mut api_params = serde_json::Map::new();
        if let Some(topics) = topics {
            api_params.insert("topics".to_string(), json!(topics));
        }
        if let Some(users) = users {
            api_params.insert("users".to_string(), json!(users));
        }
        if let Some(targets) = targets {
            api_params.insert("targets".to_string(), json!(targets));
        }
        if let Some(content) = content {
            api_params.insert("content".to_string(), json!(content));
        }
        if let Some(draft) = draft {
            api_params.insert("draft".to_string(), json!(draft));
        }
        if let Some(scheduled_at) = scheduled_at {
            api_params.insert("scheduledAt".to_string(), json!(scheduled_at));
        }

        let api_params = serde_json::Value::Object(api_params);

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

    /// Get message
    ///
    /// Get a message by its unique ID.
    ///
    pub async fn get_message(client: &Client, message_id: &str) -> Result<Message, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/messages/{}", message_id);

        let api_params = serde_json::Map::new();
        let api_params = serde_json::Value::Object(api_params);

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

    /// Delete message
    ///
    /// Delete a message. If the message is not a draft or scheduled, but has been
    /// sent, this will not recall the message.
    pub async fn delete_message(client: &Client, message_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/messages/{}", message_id);

        let api_params = serde_json::Map::new();
        let api_params = serde_json::Value::Object(api_params);

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

    /// List message logs
    ///
    /// Get the message activity logs listed by its unique ID.
    pub async fn list_message_logs(
        client: &Client,
        message_id: &str,
        queries: Option<Vec<String>>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/messages/{}/logs", message_id);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// List message targets
    ///
    /// Get a list of the targets associated with a message.
    pub async fn list_targets(
        client: &Client,
        message_id: &str,
        queries: Option<Vec<String>>,
    ) -> Result<TargetList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/messages/{}/targets", message_id);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// List providers
    ///
    /// Get a list of all providers from the current Appwrite project.
    pub async fn list_providers(
        client: &Client,
        queries: Option<Vec<String>>,
        search: Option<Vec<&str>>,
    ) -> Result<ProviderList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers";

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

    /// Create APNS provider
    ///
    /// Create a new Apple Push Notification service provider.
    pub async fn create_apns_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        auth_key: Option<&str>,
        auth_key_id: Option<&str>,
        team_id: Option<&str>,
        bundle_id: Option<&str>,
        sandbox: Option<bool>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/apns";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));
        if let Some(auth_key) = auth_key {
            api_params.insert("authKey".to_string(), json!(auth_key));
        }
        if let Some(auth_key_id) = auth_key_id {
            api_params.insert("authKeyId".to_string(), json!(auth_key_id));
        }
        if let Some(team_id) = team_id {
            api_params.insert("teamId".to_string(), json!(team_id));
        }
        if let Some(bundle_id) = bundle_id {
            api_params.insert("bundleId".to_string(), json!(bundle_id));
        }
        if let Some(sandbox) = sandbox {
            api_params.insert("sandbox".to_string(), json!(sandbox));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update APNS provider
    ///
    /// Update a Apple Push Notification service provider by its unique ID.
    pub async fn update_apns_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        enabled: Option<bool>,
        auth_key: Option<&str>,
        auth_key_id: Option<&str>,
        team_id: Option<&str>,
        bundle_id: Option<&str>,
        sandbox: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/apns/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(auth_key) = auth_key {
            api_params.insert("authKey".to_string(), json!(auth_key));
        }
        if let Some(auth_key_id) = auth_key_id {
            api_params.insert("authKeyId".to_string(), json!(auth_key_id));
        }
        if let Some(team_id) = team_id {
            api_params.insert("teamId".to_string(), json!(team_id));
        }
        if let Some(bundle_id) = bundle_id {
            api_params.insert("bundleId".to_string(), json!(bundle_id));
        }
        if let Some(sandbox) = sandbox {
            api_params.insert("sandbox".to_string(), json!(sandbox));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Create FCM provider
    ///
    /// Create a new Firebase Cloud Messaging provider.
    pub async fn create_fcm_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        service_account_json: Option<Map<String, Value>>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/fcm";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));

        if let Some(service_account_json) = service_account_json {
            api_params.insert(
                "serviceAccountJSON".to_string(),
                json!(service_account_json),
            );
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update FCM provider
    ///
    /// Update a Firebase Cloud Messaging provider by its unique ID.
    pub async fn update_fcm_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        enabled: Option<bool>,
        service_account_json: Option<Map<String, Value>>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/fcm/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(service_account_json) = service_account_json {
            api_params.insert(
                "serviceAccountJSON".to_string(),
                json!(service_account_json),
            );
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Create Mailgun provider
    ///
    /// Create a new Mailgun provider.
    pub async fn create_mail_gun_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        api_key: Option<&str>,
        domain: Option<&str>,
        is_eu_region: Option<bool>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/mailgun";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));

        // Add optional parameters if they have values
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(domain) = domain {
            api_params.insert("domain".to_string(), json!(domain));
        }
        if let Some(is_eu_region) = is_eu_region {
            api_params.insert("isEuRegion".to_string(), json!(is_eu_region));
        }
        if let Some(from_name) = from_name {
            api_params.insert("fromName".to_string(), json!(from_name));
        }
        if let Some(from_email) = from_email {
            api_params.insert("fromEmail".to_string(), json!(from_email));
        }
        if let Some(reply_to_name) = reply_to_name {
            api_params.insert("replyToName".to_string(), json!(reply_to_name));
        }
        if let Some(reply_to_email) = reply_to_email {
            api_params.insert("replyToEmail".to_string(), json!(reply_to_email));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Mailgun provider
    ///
    /// Update a Mailgun provider by its unique ID.
    pub async fn update_mail_gun_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        api_key: Option<&str>,
        domain: Option<&str>,
        is_eu_region: Option<bool>,
        enabled: Option<bool>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/mailgun/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(domain) = domain {
            api_params.insert("domain".to_string(), json!(domain));
        }
        if let Some(is_eu_region) = is_eu_region {
            api_params.insert("isEuRegion".to_string(), json!(is_eu_region));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(from_name) = from_name {
            api_params.insert("fromName".to_string(), json!(from_name));
        }
        if let Some(from_email) = from_email {
            api_params.insert("fromEmail".to_string(), json!(from_email));
        }
        if let Some(reply_to_name) = reply_to_name {
            api_params.insert("replyToName".to_string(), json!(reply_to_name));
        }
        if let Some(reply_to_email) = reply_to_email {
            api_params.insert("replyToEmail".to_string(), json!(reply_to_email));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Create Msg91 provider
    ///
    /// Create a new MSG91 provider.
    pub async fn create_msg_91_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        template_id: Option<&str>,
        sender_id: Option<&str>,
        auth_key: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/msg91";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));
        if let Some(template_id) = template_id {
            api_params.insert("templateId".to_string(), json!(template_id));
        }
        if let Some(sender_id) = sender_id {
            api_params.insert("senderId".to_string(), json!(sender_id));
        }
        if let Some(auth_key) = auth_key {
            api_params.insert("authKey".to_string(), json!(auth_key));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Msg91 provider
    ///
    /// Update a MSG91 provider by its unique ID.
    pub async fn update_msg_91_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        enabled: Option<bool>,
        template_id: Option<&str>,
        sender_id: Option<&str>,
        auth_key: Option<&str>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/msg91/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(template_id) = template_id {
            api_params.insert("templateId".to_string(), json!(template_id));
        }
        if let Some(sender_id) = sender_id {
            api_params.insert("senderId".to_string(), json!(sender_id));
        }
        if let Some(auth_key) = auth_key {
            api_params.insert("authKey".to_string(), json!(auth_key));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Create Sendgrid provider
    ///
    /// Create a new Sendgrid provider.
    pub async fn create_send_grid_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        api_key: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/sendgrid";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(from_name) = from_name {
            api_params.insert("fromName".to_string(), json!(from_name));
        }
        if let Some(from_email) = from_email {
            api_params.insert("fromEmail".to_string(), json!(from_email));
        }
        if let Some(reply_to_name) = reply_to_name {
            api_params.insert("replyToName".to_string(), json!(reply_to_name));
        }
        if let Some(reply_to_email) = reply_to_email {
            api_params.insert("replyToEmail".to_string(), json!(reply_to_email));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Sendgrid provider
    ///
    /// Update a Sendgrid provider by its unique ID.
    pub async fn update_send_grid_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        enabled: Option<bool>,
        api_key: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/sendgrid/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(from_name) = from_name {
            api_params.insert("fromName".to_string(), json!(from_name));
        }
        if let Some(from_email) = from_email {
            api_params.insert("fromEmail".to_string(), json!(from_email));
        }
        if let Some(reply_to_name) = reply_to_name {
            api_params.insert("replyToName".to_string(), json!(reply_to_name));
        }
        if let Some(reply_to_email) = reply_to_email {
            api_params.insert("replyToEmail".to_string(), json!(reply_to_email));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Create SMTP provider
    ///
    /// Create a new SMTP provider.
    pub async fn create_smtp_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        host: &str,
        port: Option<i32>,
        username: Option<&str>,
        password: Option<&str>,
        encryption: Option<SmtpEncryption>,
        auto_tls: Option<bool>,
        mailer: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/smtp";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));
        api_params.insert("host".to_string(), json!(host));
        if let Some(port) = port {
            api_params.insert("port".to_string(), json!(port));
        }
        if let Some(username) = username {
            api_params.insert("username".to_string(), json!(username));
        }
        if let Some(password) = password {
            api_params.insert("password".to_string(), json!(password));
        }
        if let Some(encryption) = encryption {
            api_params.insert("encryption".to_string(), json!(encryption));
        }
        if let Some(auto_tls) = auto_tls {
            api_params.insert("autoTLS".to_string(), json!(auto_tls));
        }
        if let Some(mailer) = mailer {
            api_params.insert("mailer".to_string(), json!(mailer));
        }
        if let Some(from_name) = from_name {
            api_params.insert("fromName".to_string(), json!(from_name));
        }
        if let Some(from_email) = from_email {
            api_params.insert("fromEmail".to_string(), json!(from_email));
        }
        if let Some(reply_to_name) = reply_to_name {
            api_params.insert("replyToName".to_string(), json!(reply_to_name));
        }
        if let Some(reply_to_email) = reply_to_email {
            api_params.insert("replyToEmail".to_string(), json!(reply_to_email));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update SMTP provider
    ///
    /// Update a SMTP provider by its unique ID.
    pub async fn update_smtp_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        host: Option<&str>,
        port: Option<i32>,
        username: Option<&str>,
        password: Option<&str>,
        encryption: Option<SmtpEncryption>,
        auto_tls: Option<bool>,
        mailer: Option<&str>,
        from_name: Option<&str>,
        from_email: Option<&str>,
        reply_to_name: Option<&str>,
        reply_to_email: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/smtp/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(host) = host {
            api_params.insert("host".to_string(), json!(host));
        }
        if let Some(port) = port {
            api_params.insert("port".to_string(), json!(port));
        }
        if let Some(username) = username {
            api_params.insert("username".to_string(), json!(username));
        }
        if let Some(password) = password {
            api_params.insert("password".to_string(), json!(password));
        }
        if let Some(encryption) = encryption {
            api_params.insert("encryption".to_string(), json!(encryption));
        }
        if let Some(auto_tls) = auto_tls {
            api_params.insert("autoTLS".to_string(), json!(auto_tls));
        }
        if let Some(mailer) = mailer {
            api_params.insert("mailer".to_string(), json!(mailer));
        }
        if let Some(from_name) = from_name {
            api_params.insert("fromName".to_string(), json!(from_name));
        }
        if let Some(from_email) = from_email {
            api_params.insert("fromEmail".to_string(), json!(from_email));
        }
        if let Some(reply_to_name) = reply_to_name {
            api_params.insert("replyToName".to_string(), json!(reply_to_name));
        }
        if let Some(reply_to_email) = reply_to_email {
            api_params.insert("replyToEmail".to_string(), json!(reply_to_email));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Create Telesign provider
    ///
    /// Create a new Telesign provider.
    pub async fn create_telesign_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        from: Option<&str>,
        customer_id: Option<&str>,
        api_key: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/telesign";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));
        if let Some(from) = from {
            api_params.insert("from".to_string(), json!(from));
        }
        if let Some(customer_id) = customer_id {
            api_params.insert("customerId".to_string(), json!(customer_id));
        }
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Telesign provider
    ///
    /// Update a Telesign provider by its unique ID.
    pub async fn update_telesign_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        enabled: Option<bool>,
        customer_id: Option<&str>,
        api_key: Option<&str>,
        from: Option<&str>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/telesign/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(customer_id) = customer_id {
            api_params.insert("customerId".to_string(), json!(customer_id));
        }
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(from) = from {
            api_params.insert("from".to_string(), json!(from));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Create Textmagic provider
    ///
    /// Create a new Textmagic provider.
    pub async fn create_text_magic_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        from: Option<&str>,
        username: Option<&str>,
        api_key: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/textmagic";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));
        if let Some(from) = from {
            api_params.insert("from".to_string(), json!(from));
        }
        if let Some(username) = username {
            api_params.insert("username".to_string(), json!(username));
        }
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Textmagic provider
    ///
    /// Update a Textmagic provider by its unique ID.
    pub async fn update_text_magic_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        enabled: Option<bool>,
        username: Option<&str>,
        api_key: Option<&str>,
        from: Option<&str>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/textmagic/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(username) = username {
            api_params.insert("username".to_string(), json!(username));
        }
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(from) = from {
            api_params.insert("from".to_string(), json!(from));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Create Twilio provider
    ///
    /// Create a new Twilio provider.
    pub async fn create_twilio_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        from: Option<&str>,
        account_sid: Option<&str>,
        auth_token: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/twilio";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));
        if let Some(from) = from {
            api_params.insert("from".to_string(), json!(from));
        }
        if let Some(account_sid) = account_sid {
            api_params.insert("accountSid".to_string(), json!(account_sid));
        }
        if let Some(auth_token) = auth_token {
            api_params.insert("authToken".to_string(), json!(auth_token));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Twilio provider
    ///
    /// Update a Twilio provider by its unique ID.
    pub async fn update_twilio_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        enabled: Option<bool>,
        account_sid: Option<&str>,
        auth_token: Option<&str>,
        from: Option<&str>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/twilio/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(account_sid) = account_sid {
            api_params.insert("accountSid".to_string(), json!(account_sid));
        }
        if let Some(auth_token) = auth_token {
            api_params.insert("authToken".to_string(), json!(auth_token));
        }
        if let Some(from) = from {
            api_params.insert("from".to_string(), json!(from));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Create Vonage provider
    ///
    /// Create a new Vonage provider.
    pub async fn create_vonage_provider(
        client: &Client,
        provider_id: &str,
        name: &str,
        from: Option<&str>,
        api_key: Option<&str>,
        api_secret: Option<&str>,
        enabled: Option<bool>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/vonage";

        let mut api_params = serde_json::Map::new();
        api_params.insert("providerId".to_string(), json!(provider_id));
        api_params.insert("name".to_string(), json!(name));
        if let Some(from) = from {
            api_params.insert("from".to_string(), json!(from));
        }
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(api_secret) = api_secret {
            api_params.insert("apiSecret".to_string(), json!(api_secret));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Vonage provider
    ///
    /// Update a Vonage provider by its unique ID.
    pub async fn update_vonage_provider(
        client: &Client,
        provider_id: &str,
        name: Option<&str>,
        enabled: Option<bool>,
        api_key: Option<&str>,
        api_secret: Option<&str>,
        from: Option<&str>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/vonage/{}", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(enabled) = enabled {
            api_params.insert("enabled".to_string(), json!(enabled));
        }
        if let Some(api_key) = api_key {
            api_params.insert("apiKey".to_string(), json!(api_key));
        }
        if let Some(api_secret) = api_secret {
            api_params.insert("apiSecret".to_string(), json!(api_secret));
        }
        if let Some(from) = from {
            api_params.insert("from".to_string(), json!(from));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Get provider
    ///
    /// Get a provider by its unique ID.
    ///
    pub async fn get_provider(client: &Client, provider_id: &str) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/{}", provider_id);

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

    /// Delete provider
    ///
    /// Delete a provider by its unique ID.
    pub async fn delete_provider(client: &Client, provider_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/{}", provider_id);

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

    /// List provider logs
    ///
    /// Get the provider activity logs listed by its unique ID.
    pub async fn list_provider_logs(
        client: &Client,
        provider_id: &str,
        queries: Option<Vec<String>>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/{}/logs", provider_id);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// List subscriber logs
    ///
    /// Get the subscriber activity logs listed by its unique ID.
    pub async fn list_subscriber_logs(
        client: &Client,
        subscriber_id: &str,
        queries: Option<Vec<String>>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/subscribers/{}/logs", subscriber_id);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// List topics
    ///
    /// Get a list of all topics from the current Appwrite project.
    pub async fn list_topics(
        client: &Client,
        queries: Option<Vec<String>>,
        search: Option<String>,
    ) -> Result<TopicList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/topics";

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

    /// Create topic
    ///
    /// Create a new topic.
    pub async fn create_topics(
        client: &Client,
        topic_id: &str,
        name: &str,
        subscribe: Option<Vec<&str>>,
    ) -> Result<Topic, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/topics";

        let mut api_params = serde_json::Map::new();
        api_params.insert("topicId".to_string(), json!(topic_id));
        api_params.insert("name".to_string(), json!(name));
        if let Some(subscribe) = subscribe {
            api_params.insert("subscribe".to_string(), json!(subscribe));
        }
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get topic
    ///
    /// Get a topic by its unique ID.
    ///
    pub async fn get_topic(client: &Client, topic_id: &str) -> Result<Topic, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}", topic_id);

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

    /// Update topic
    ///
    /// Update a topic by its unique ID.
    ///
    pub async fn update_topic(
        client: &Client,
        topic_id: &str,
        name: Option<&str>,
        subscribe: Option<Vec<&str>>,
    ) -> Result<Topic, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}", topic_id);

        let mut api_params = serde_json::Map::new();
        if let Some(name) = name {
            api_params.insert("name".to_string(), json!(name));
        }
        if let Some(subscribe) = subscribe {
            api_params.insert("subscribe".to_string(), json!(subscribe));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// Delete topic
    ///
    /// Delete a topic by its unique ID.
    pub async fn delete_topic(client: &Client, topic_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}", topic_id);

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

    /// List topic logs
    ///
    /// Get the topic activity logs listed by its unique ID.
    pub async fn list_topic_logs(
        client: &Client,
        topic_id: &str,
        queries: Option<Vec<String>>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}/logs", topic_id);

        let mut api_params = serde_json::Map::new();
        if let Some(queries) = queries {
            api_params.insert("queries".to_string(), json!(queries));
        }
        let api_params = serde_json::Value::Object(api_params);

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

    /// List subscribers
    ///
    /// Get a list of all subscribers from the current Appwrite project.
    pub async fn list_subscribers(
        client: &Client,
        topic_id: &str,
        queries: Option<Vec<String>>,
        search: Option<String>,
    ) -> Result<SubscriberList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}/subscribers", topic_id);

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

    /// Create subscriber
    ///
    /// Create a new subscriber.
    pub async fn create_subscriber(
        client: &Client,
        topic_id: &str,
        subscriber_id: &str,
        target_id: &str,
    ) -> Result<Subscriber, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}/subscribers", topic_id);

        let mut api_params = serde_json::Map::new();
        api_params.insert("subscriberId".to_string(), json!(subscriber_id));
        api_params.insert("targetId".to_string(), json!(target_id));
        let api_params = serde_json::Value::Object(api_params);

        let mut api_headers = header::HeaderMap::new();
        api_headers.insert(header::CONTENT_TYPE, "application/json".parse()?);

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

    /// Get subscriber
    ///
    /// Get a subscriber by its unique ID.
    ///
    pub async fn get_subscriber(
        client: &Client,
        topic_id: &str,
        subscriber_id: &str,
    ) -> Result<Subscriber, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!(
            "/messaging/topics/{}/subscribers/{}",
            topic_id, subscriber_id
        );

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

    // Delete subscriber
    ///
    /// Delete a subscriber by its unique ID.
    pub async fn delete_subscriber(
        client: &Client,
        topic_id: &str,
        subscriber_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!(
            "/messaging/topics/{}/subscribers/{}",
            topic_id, subscriber_id
        );

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
}
