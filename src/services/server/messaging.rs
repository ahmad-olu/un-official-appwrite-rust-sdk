//! # Messaging
//!
//! The Messaging service allows you to send messages to any provider type
//! (SMTP, push notification, SMS, etc.).

use std::collections::HashMap;

use serde_json::Value;

use crate::{
    app_json_header,
    client::Client,
    enumm::HttpMethod,
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
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list_messages(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<MessageList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/messages";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create email
    ///
    /// Create a new email message.
    ///* messageId => string
    ///* subject => string
    ///* content => string
    ///* topics => vec(string)?
    ///* users => vec(string)?
    ///* targets => vec(string)?
    ///* bcc => vec(string)?
    ///* cc => vec(string)?
    ///* attachments => vec(string)?
    ///* draft => bool?
    ///* html => bool?
    ///* scheduledAt => bool?
    pub async fn create_email(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Message, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/messages/email";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update email
    ///
    /// Update an email message by its unique ID.
    ///
    ///* subject => string
    ///* content => string
    ///* topics => vec(string)?
    ///* users => vec(string)?
    ///* targets => vec(string)?
    ///* bcc => vec(string)?
    ///* cc => vec(string)?
    ///* draft => bool?
    ///* html => bool?
    ///* scheduledAt => bool?
    pub async fn update_email(
        client: &Client,
        message_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Message, Error> {
        //let api_path = "/messaging/messages/email";
        let api_path = "/messaging/messages/email/{messageId}".replace("{messageId}", message_id);

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

    /// Create push notification
    ///
    /// Create a new push notification.
    ///* messageId => string
    ///* title => string
    ///* body => string
    ///* topics => vec(string)?
    ///* users => vec(string)?
    ///* targets => vec(string?)
    ///* data => HashMap<String, Value>?
    ///* action => string?
    ///* image => string?
    ///* icon => string?
    ///* sound => string?
    ///* color => string?
    ///* tag => string?
    ///* badge => string?
    ///* draft => bool?
    ///* scheduledAt => string?
    pub async fn create_push(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Message, Error> {
        let api_path = "/messaging/messages/push";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update push notification
    ///
    /// Update a push notification by its unique ID.
    ///
    ///* title => string?
    ///* body => string?
    ///* topics => vec(string)?
    ///* users => vec(string)?
    ///* targets => vec(string?)
    ///* data => HashMap<String, Value>?
    ///* action => string?
    ///* image => string?
    ///* icon => string?
    ///* sound => string?
    ///* color => string?
    ///* tag => string?
    ///* badge => string?
    ///* draft => bool?
    ///* scheduledAt => string?
    pub async fn update_push(
        client: &Client,
        message_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Message, Error> {
        let api_path = format!("/messaging/messages/push/{}", message_id);

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

    /// Create SMS
    ///
    /// Create a new SMS message.
    ///* messageId => string
    ///* content => string
    ///* topics => vec(string)?
    ///* users => vec(string)?
    ///* targets => vec(string)?
    ///* draft => bool?
    ///* scheduleAt => string?
    pub async fn create_sms(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Message, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/messages/sms";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update SMS
    ///
    /// Update an email message by its unique ID.
    ///
    ///* topics => vec(string)?
    ///* users => vec(string)?
    ///* targets => vec(string)?
    ///* content => string?
    ///* draft => bool?
    ///* scheduleAt => string?
    pub async fn update_sms(
        client: &Client,
        message_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Message, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/messages/sms/{}", message_id);

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

    /// Get message
    ///
    /// Get a message by its unique ID.
    ///
    pub async fn get_message(client: &Client, message_id: &str) -> Result<Message, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/messages/{}", message_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
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

    /// List message logs
    ///
    /// Get the message activity logs listed by its unique ID.
    ///* queries => vec(string)?
    pub async fn list_message_logs(
        client: &Client,
        message_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/messages/{}/logs", message_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List message targets
    ///
    /// Get a list of the targets associated with a message.
    ///* queries => vec(string)?
    pub async fn list_targets(
        client: &Client,
        message_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<TargetList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/messages/{}/targets", message_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List providers
    ///
    /// Get a list of all providers from the current Appwrite project.
    ///* queries => vec(string)?
    ///* search => vec(string)?
    pub async fn list_providers(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<ProviderList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create APNS provider
    ///
    /// Create a new Apple Push Notification service provider.
    ///* providerId => string
    ///* name => string
    ///* authKey => string?
    ///* authKeyId => string?
    ///* teamId => string?
    ///* bundleId => string?
    ///* sandbox => bool?
    ///* enabled => bool?
    pub async fn create_apns_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/apns";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update APNS provider
    ///
    /// Update a Apple Push Notification service provider by its unique ID.
    ///* name => string?
    ///* authKey => string?
    ///* authKeyId => string?
    ///* teamId => string?
    ///* bundleId => string?
    ///* sandbox => bool?
    ///* enabled => bool?
    pub async fn update_apns_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/apns/{}", provider_id);

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

    /// Create FCM provider
    ///
    /// Create a new Firebase Cloud Messaging provider.
    ///* providerId => string
    ///* name => string
    ///* serviceAccountJSON => HashMap<String, String>?
    ///* enabled => bool?
    pub async fn create_fcm_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/fcm";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update FCM provider
    ///
    /// Update a Firebase Cloud Messaging provider by its unique ID.
    ///* name => string?
    ///* serviceAccountJSON => HashMap<String, Value>?
    ///* enabled => bool?
    pub async fn update_fcm_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/fcm/{}", provider_id);

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

    /// Create Mailgun provider
    ///
    /// Create a new Mailgun provider.
    ///* providerId => string
    ///* name => string
    ///* apiKey => string?
    ///* domain => string?
    ///* isEuRegion => bool?
    ///* fromName => string?
    ///* fromEmail => string?
    ///* replyToName => string?
    ///* replyToEmail => string?
    ///* enabled => bool?
    pub async fn create_mail_gun_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/mailgun";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Mailgun provider
    ///
    /// Update a Mailgun provider by its unique ID.
    ///* name => string?
    ///* apiKey => string?
    ///* domain => string?
    ///* isEuRegion => bool?
    ///* fromName => string?
    ///* fromEmail => string?
    ///* replyToName => string?
    ///* replyToEmail => string?
    ///* enabled => bool?
    pub async fn update_mail_gun_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/mailgun/{}", provider_id);

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

    /// Create Msg91 provider
    ///
    /// Create a new MSG91 provider.
    ///* providerId => string
    ///* name => string
    ///* templateId => string?
    ///* senderId => string?h
    ///* authKey => string?
    ///* enabled => bool?
    pub async fn create_msg_91_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/msg91";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Msg91 provider
    ///
    /// Update a MSG91 provider by its unique ID.
    ///* name => string?
    ///* templateId => string?
    ///* senderId => string?h
    ///* authKey => string?
    ///* enabled => bool?
    pub async fn update_msg_91_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/msg91/{}", provider_id);

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

    /// Create Sendgrid provider
    ///
    /// Create a new Sendgrid provider.
    ///* providerId => string
    ///* name => string
    ///* apiKey => string?
    ///* fromName => string?
    ///* fromEmail => string?
    ///* replyToName => string?
    ///* replyToEmail => string?
    ///* enabled => bool?
    pub async fn create_send_grid_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/sendgrid";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Sendgrid provider
    ///
    /// Update a Sendgrid provider by its unique ID.
    ///* name => string?
    ///* apiKey => string?
    ///* fromName => string?
    ///* fromEmail => string?
    ///* replyToName => string?
    ///* replyToEmail => string?
    ///* enabled => bool?
    pub async fn update_send_grid_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/sendgrid/{}", provider_id);

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

    /// Create SMTP provider
    ///
    /// Create a new SMTP provider.
    ///* providerId => string
    ///* name => string
    ///* host => string
    ///* port => number?
    ///* username => string?
    ///* password => string?
    ///* encryption => SmtpEncryption?
    ///* autoTLS => bool?
    ///* mailer => string?
    ///* fromName => string?
    ///* fromEmail => string?
    ///* replyToName => string?
    ///* replyToEmail => string?
    ///* enabled => bool?
    pub async fn create_smtp_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/smtp";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update SMTP provider
    ///
    /// Update a SMTP provider by its unique ID.
    ///* name => string?
    ///* host => string?
    ///* port => number?
    ///* username => string?
    ///* password => string?
    ///* encryption => SmtpEncryption?
    ///* autoTLS => bool?
    ///* mailer => string?
    ///* fromName => string?
    ///* fromEmail => string?
    ///* replyToName => string?
    ///* replyToEmail => string?
    ///* enabled => bool?
    pub async fn update_smtp_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/smtp/{}", provider_id);

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

    /// Create Telesign provider
    ///
    /// Create a new Telesign provider.
    ///* providerId => string
    ///* name => string
    ///* from => string?
    ///* customId => string?
    ///* apiKey => string?
    ///* enabled => bool?
    pub async fn create_telesign_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/telesign";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Telesign provider
    ///
    /// Update a Telesign provider by its unique ID.
    ///* name => string?
    ///* from => string?
    ///* customId => string?
    ///* apiKey => string?
    ///* enabled => bool?
    pub async fn update_telesign_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/telesign/{}", provider_id);

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

    /// Create Textmagic provider
    ///
    /// Create a new Textmagic provider.
    ///* providerId => string
    ///* name => string
    ///* from => string?
    ///* username => string?
    ///* apiKey => string?
    ///* enabled => bool?
    pub async fn create_text_magic_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/textmagic";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Textmagic provider
    ///
    /// Update a Textmagic provider by its unique ID.
    ///* name => string
    ///* from => string?
    ///* username => string?
    ///* apiKey => string?
    ///* enabled => bool?
    pub async fn update_text_magic_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/textmagic/{}", provider_id);

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

    /// Create Twilio provider
    ///
    /// Create a new Twilio provider.
    ///* providerId => string
    ///* name => string
    ///* from => string?
    ///* accountSid => string? //TODO! check spelling
    ///* authToken => string?
    ///* enabled => bool?
    pub async fn create_twilio_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/twilio";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Twilio provider
    ///
    /// Update a Twilio provider by its unique ID.
    ///* name => string?
    ///* from => string?
    ///* accountSid => string? //TODO! check spelling
    ///* authToken => string?
    ///* enabled => bool?
    pub async fn update_twilio_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/twilio/{}", provider_id);

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

    /// Create Vonage provider
    ///
    /// Create a new Vonage provider.
    ///* providerId => string
    ///* name => string
    ///* from => string?
    ///* apiKey => string?
    ///* apiSecret => string?
    ///* enabled => bool?
    pub async fn create_vonage_provider(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/providers/vonage";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update Vonage provider
    ///
    /// Update a Vonage provider by its unique ID.
    ///* name => string?
    ///* from => string?
    ///* apiKey => string?
    ///* apiSecret => string?
    ///* enabled => bool?
    pub async fn update_vonage_provider(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/vonage/{}", provider_id);

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

    /// Get provider
    ///
    /// Get a provider by its unique ID.
    ///
    pub async fn get_provider(client: &Client, provider_id: &str) -> Result<Provider, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/{}", provider_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete provider
    ///
    /// Delete a provider by its unique ID.
    pub async fn delete_provider(client: &Client, provider_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/{}", provider_id);

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

    /// List provider logs
    ///
    /// Get the provider activity logs listed by its unique ID.
    ///* queries => vec(string)?
    pub async fn list_provider_logs(
        client: &Client,
        provider_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/providers/{}/logs", provider_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List subscriber logs
    ///
    /// Get the subscriber activity logs listed by its unique ID.
    ///* queries => vec(string)?
    pub async fn list_subscriber_logs(
        client: &Client,
        subscriber_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/subscribers/{}/logs", subscriber_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List topics
    ///
    /// Get a list of all topics from the current Appwrite project.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list_topics(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<TopicList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/topics";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create topic
    ///
    /// Create a new topic.
    ///* topicId => string
    ///* name => string
    ///* subscribe => vec(string)?
    pub async fn create_topics(
        client: &Client,
        args: HashMap<String, Value>,
    ) -> Result<Topic, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/messaging/topics";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, args, None)
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update topic
    ///
    /// Update a topic by its unique ID.
    ///
    ///* name => string?
    ///* subscribe => vec(string)?
    pub async fn update_topic(
        client: &Client,
        topic_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Topic, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}", topic_id);

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

    /// Delete topic
    ///
    /// Delete a topic by its unique ID.
    pub async fn delete_topic(client: &Client, topic_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}", topic_id);

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

    /// List topic logs
    ///
    /// Get the topic activity logs listed by its unique ID.
    ///* queries => vec(string)?
    pub async fn list_topic_logs(
        client: &Client,
        topic_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<LogList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}/logs", topic_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// List subscribers
    ///
    /// Get a list of all subscribers from the current Appwrite project.
    ///* search => string?
    ///* queries => vec(string)?
    pub async fn list_subscribers(
        client: &Client,
        topic_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<SubscriberList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}/subscribers", topic_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create subscriber
    ///
    /// Create a new subscriber.
    ///* subscriberId => string
    ///* targetId => string
    pub async fn create_subscriber(
        client: &Client,
        topic_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Subscriber, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = format!("/messaging/topics/{}/subscribers", topic_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path.as_str(), api_headers, args, None)
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

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, args, None)
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
}
