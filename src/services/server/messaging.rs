// //! # Messaging
// //!
// //! The Messaging service allows you to send messages to any provider type
// //! (SMTP, push notification, SMS, etc.).

// use serde_json::{Map, Value};

// use crate::{
//     api_params, app_json_header,
//     client::Client,
//     enumm::HttpMethod,
//     enums::smtp_encryption::SmtpEncryption,
//     error::Error,
//     models::{
//         log_list::LogList, message::Message, message_list::MessageList, provider::Provider,
//         provider_list::ProviderList, subscriber::Subscriber, subscriber_list::SubscriberList,
//         target_list::TargetList, topic::Topic, topic_list::TopicList,
//     },
// };

// pub struct Messaging;

// impl Messaging {
//     /// List messages
//     ///
//     /// Get a list of all messages from the current Appwrite project.
//     pub async fn list_messages(
//         client: &Client,
//         queries: Option<Vec<String>>,
//         search: Option<String>,
//     ) -> Result<MessageList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/messages";

//         let api_params = api_params!(
//             "queries"=> queries,
//             "search"=> search,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create email
//     ///
//     /// Create a new email message.
//     pub async fn create_email(
//         client: &Client,
//         message_id: &str,
//         subject: &str,
//         content: &str,
//         topics: Option<Vec<&str>>,
//         users: Option<Vec<&str>>,
//         targets: Option<Vec<&str>>,
//         cc: Option<Vec<&str>>,
//         bcc: Option<Vec<&str>>,
//         attachments: Option<Vec<&str>>,
//         draft: Option<bool>,
//         html: Option<bool>,
//         scheduled_at: Option<bool>,
//     ) -> Result<Message, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/messages/email";

//         let api_params = api_params!(
//             "messageId"=> Some(message_id),
//             "subject"=> Some(subject),
//             "content"=>Some(content),
//             "topics"=> topics,
//             "users"=> users,
//             "targets"=> targets,
//             "cc"=>cc,
//             "bcc"=> bcc,
//             "attachments"=> attachments,
//             "draft"=> draft,
//             "html"=> html,
//             "scheduleAt"=> scheduled_at,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update email
//     ///
//     /// Update an email message by its unique ID.
//     ///
//     pub async fn update_email(
//         client: &Client,
//         message_id: &str,
//         topics: Option<Vec<&str>>,
//         users: Option<Vec<&str>>,
//         targets: Option<Vec<&str>>,
//         subject: Option<&str>,
//         content: Option<&str>,
//         draft: Option<bool>,
//         html: Option<bool>,
//         cc: Option<Vec<&str>>,
//         bcc: Option<Vec<&str>>,
//         scheduled_at: Option<&str>,
//     ) -> Result<Message, Error> {
//         //let api_path = "/messaging/messages/email";
//         let api_path = "/messaging/messages/email/{messageId}".replace("{messageId}", message_id);

//         let api_params = api_params!(
//             "topics"=> topics,
//             "users"=> users,
//             "targets"=> targets,
//             "subject"=> subject,
//             "content"=>content,
//             "draft"=> draft,
//             "html"=> html,
//             "cc"=>cc,
//             "bcc"=> bcc,
//             "scheduleAt"=> scheduled_at,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create push notification
//     ///
//     /// Create a new push notification.
//     pub async fn create_push(
//         client: &Client,
//         message_id: &str,
//         title: &str,
//         body: &str,
//         topics: Option<Vec<&str>>,
//         users: Option<Vec<&str>>,
//         targets: Option<Vec<&str>>,
//         data: Option<Map<String, Value>>,
//         action: Option<&str>,
//         image: Option<&str>,
//         icon: Option<&str>,
//         sound: Option<&str>,
//         color: Option<&str>,
//         tag: Option<&str>,
//         badge: Option<&str>,
//         draft: Option<bool>,
//         scheduled_at: Option<&str>,
//     ) -> Result<Message, Error> {
//         let api_path = "/messaging/messages/push";

//         let api_params = api_params!(
//             "messageId"=> Some(message_id),
//             "title"=> Some(title),
//             "body"=> Some(body),
//             "topics"=> topics,
//             "users"=>users,
//             "targets"=> targets,
//             "data"=> data,
//             "action"=>action,
//             "image"=> image,
//             "icon"=> icon,
//             "sound"=> sound,
//             "color"=> color,
//             "tag"=> tag,
//             "badge"=> badge,
//             "draft"=> draft,
//             "scheduleAt"=> scheduled_at,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update push notification
//     ///
//     /// Update a push notification by its unique ID.
//     ///
//     pub async fn update_push(
//         client: &Client,
//         message_id: &str,
//         topics: Option<Vec<&str>>,
//         users: Option<Vec<&str>>,
//         targets: Option<Vec<&str>>,
//         title: Option<&str>,
//         body: Option<&str>,
//         data: Option<Map<String, Value>>,
//         action: Option<&str>,
//         image: Option<&str>,
//         icon: Option<&str>,
//         sound: Option<&str>,
//         color: Option<&str>,
//         tag: Option<&str>,
//         badge: Option<i32>,
//         draft: Option<bool>,
//         scheduled_at: Option<&str>,
//     ) -> Result<Message, Error> {
//         let api_path = format!("/messaging/messages/push/{}", message_id);

//         let api_params = api_params!(
//             "topics"=> topics,
//             "users"=> users,
//             "targets"=> targets,
//             "title"=>title,
//             "body"=> body,
//             "data"=> data,
//             "action"=>action,
//             "image"=> image,
//             "icon"=> icon,
//             "sound"=> sound,
//             "color"=> color,
//             "tag"=> tag,
//             "badge"=> badge,
//             "draft"=> draft,
//             "scheduleAt"=> scheduled_at,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create SMS
//     ///
//     /// Create a new SMS message.
//     pub async fn create_sms(
//         client: &Client,
//         message_id: &str,
//         content: &str,
//         topics: Option<Vec<&str>>,
//         users: Option<Vec<&str>>,
//         targets: Option<Vec<&str>>,
//         draft: Option<bool>,
//         scheduled_at: Option<&str>,
//     ) -> Result<Message, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/messages/sms";

//         let api_params = api_params!(
//             "messageId"=> Some(message_id),
//             "content"=> Some(content),
//             "topics"=> topics,
//             "users"=>users,
//             "targets"=> targets,
//             "draft"=> draft,
//             "scheduleAt"=> scheduled_at,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update SMS
//     ///
//     /// Update an email message by its unique ID.
//     ///
//     pub async fn update_sms(
//         client: &Client,
//         message_id: &str,
//         topics: Option<Vec<&str>>,
//         users: Option<Vec<&str>>,
//         targets: Option<Vec<&str>>,
//         content: Option<&str>,
//         draft: Option<bool>,
//         scheduled_at: Option<&str>,
//     ) -> Result<Message, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/messages/sms/{}", message_id);

//         let api_params = api_params!(
//             "topics"=> topics,
//             "users"=>users,
//             "targets"=> targets,
//             "content"=> content,
//             "draft"=> draft,
//             "scheduleAt"=> scheduled_at,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get message
//     ///
//     /// Get a message by its unique ID.
//     ///
//     pub async fn get_message(client: &Client, message_id: &str) -> Result<Message, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/messages/{}", message_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete message
//     ///
//     /// Delete a message. If the message is not a draft or scheduled, but has been
//     /// sent, this will not recall the message.
//     pub async fn delete_message(client: &Client, message_id: &str) -> Result<(), Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/messages/{}", message_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }

//     /// List message logs
//     ///
//     /// Get the message activity logs listed by its unique ID.
//     pub async fn list_message_logs(
//         client: &Client,
//         message_id: &str,
//         queries: Option<Vec<String>>,
//     ) -> Result<LogList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/messages/{}/logs", message_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List message targets
//     ///
//     /// Get a list of the targets associated with a message.
//     pub async fn list_targets(
//         client: &Client,
//         message_id: &str,
//         queries: Option<Vec<String>>,
//     ) -> Result<TargetList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/messages/{}/targets", message_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List providers
//     ///
//     /// Get a list of all providers from the current Appwrite project.
//     pub async fn list_providers(
//         client: &Client,
//         queries: Option<Vec<String>>,
//         search: Option<Vec<&str>>,
//     ) -> Result<ProviderList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers";

//         let api_params = api_params!(
//             "queries"=> queries,
//             "search"=>search,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create APNS provider
//     ///
//     /// Create a new Apple Push Notification service provider.
//     pub async fn create_apns_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         auth_key: Option<&str>,
//         auth_key_id: Option<&str>,
//         team_id: Option<&str>,
//         bundle_id: Option<&str>,
//         sandbox: Option<bool>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/apns";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=>Some(name),
//             "authKey"=> auth_key,
//             "authKeyId"=> auth_key_id,
//             "teamId"=> team_id,
//             "bundleId"=> bundle_id,
//             "sandbox"=> sandbox,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update APNS provider
//     ///
//     /// Update a Apple Push Notification service provider by its unique ID.
//     pub async fn update_apns_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         enabled: Option<bool>,
//         auth_key: Option<&str>,
//         auth_key_id: Option<&str>,
//         team_id: Option<&str>,
//         bundle_id: Option<&str>,
//         sandbox: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/apns/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "authKey"=> auth_key,
//             "authKeyId"=> auth_key_id,
//             "teamId"=> team_id,
//             "bundleId"=> bundle_id,
//             "sandbox"=> sandbox,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create FCM provider
//     ///
//     /// Create a new Firebase Cloud Messaging provider.
//     pub async fn create_fcm_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         service_account_json: Option<Map<String, Value>>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/fcm";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=> Some(name),
//             "serviceAccountJSON"=> service_account_json,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update FCM provider
//     ///
//     /// Update a Firebase Cloud Messaging provider by its unique ID.
//     pub async fn update_fcm_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         enabled: Option<bool>,
//         service_account_json: Option<Map<String, Value>>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/fcm/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "serviceAccountJSON"=> service_account_json,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create Mailgun provider
//     ///
//     /// Create a new Mailgun provider.
//     pub async fn create_mail_gun_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         api_key: Option<&str>,
//         domain: Option<&str>,
//         is_eu_region: Option<bool>,
//         from_name: Option<&str>,
//         from_email: Option<&str>,
//         reply_to_name: Option<&str>,
//         reply_to_email: Option<&str>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/mailgun";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=> Some(name),
//             "apiKey"=> api_key,
//             "domain"=> domain,
//             "isEuRegion"=> is_eu_region,
//             "fromName"=> from_name,
//             "fromEmail"=> from_email,
//             "replyToName"=>reply_to_name,
//             "replyToEmail"=> reply_to_email,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update Mailgun provider
//     ///
//     /// Update a Mailgun provider by its unique ID.
//     pub async fn update_mail_gun_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         api_key: Option<&str>,
//         domain: Option<&str>,
//         is_eu_region: Option<bool>,
//         enabled: Option<bool>,
//         from_name: Option<&str>,
//         from_email: Option<&str>,
//         reply_to_name: Option<&str>,
//         reply_to_email: Option<&str>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/mailgun/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "apiKey"=> api_key,
//             "domain"=> domain,
//             "isEuRegion"=> is_eu_region,
//             "fromName"=> from_name,
//             "fromEmail"=> from_email,
//             "replyToName"=>reply_to_name,
//             "replyToEmail"=> reply_to_email,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create Msg91 provider
//     ///
//     /// Create a new MSG91 provider.
//     pub async fn create_msg_91_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         template_id: Option<&str>,
//         sender_id: Option<&str>,
//         auth_key: Option<&str>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/msg91";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=> Some(name),
//             "templateId"=> template_id,
//             "senderId"=> sender_id,
//             "authKey"=> auth_key,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update Msg91 provider
//     ///
//     /// Update a MSG91 provider by its unique ID.
//     pub async fn update_msg_91_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         enabled: Option<bool>,
//         template_id: Option<&str>,
//         sender_id: Option<&str>,
//         auth_key: Option<&str>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/msg91/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "templateId"=> template_id,
//             "senderId"=> sender_id,
//             "authKey"=> auth_key,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create Sendgrid provider
//     ///
//     /// Create a new Sendgrid provider.
//     pub async fn create_send_grid_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         api_key: Option<&str>,
//         from_name: Option<&str>,
//         from_email: Option<&str>,
//         reply_to_name: Option<&str>,
//         reply_to_email: Option<&str>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/sendgrid";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=> Some(name),
//             "apiKey"=> api_key,
//             "fromName"=> from_name,
//             "fromEmail"=> from_email,
//             "replyToName"=> reply_to_name,
//             "replyToEmail"=> reply_to_email,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update Sendgrid provider
//     ///
//     /// Update a Sendgrid provider by its unique ID.
//     pub async fn update_send_grid_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         enabled: Option<bool>,
//         api_key: Option<&str>,
//         from_name: Option<&str>,
//         from_email: Option<&str>,
//         reply_to_name: Option<&str>,
//         reply_to_email: Option<&str>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/sendgrid/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "apiKey"=> api_key,
//             "fromName"=> from_name,
//             "fromEmail"=> from_email,
//             "replyToName"=> reply_to_name,
//             "replyToEmail"=> reply_to_email,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create SMTP provider
//     ///
//     /// Create a new SMTP provider.
//     pub async fn create_smtp_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         host: &str,
//         port: Option<i32>,
//         username: Option<&str>,
//         password: Option<&str>,
//         encryption: Option<SmtpEncryption>,
//         auto_tls: Option<bool>,
//         mailer: Option<&str>,
//         from_name: Option<&str>,
//         from_email: Option<&str>,
//         reply_to_name: Option<&str>,
//         reply_to_email: Option<&str>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/smtp";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=> Some(name),
//             "host"=> Some(host),
//             "port"=> port,
//             "username"=> username,
//             "password"=> password,
//             "encryption"=> encryption,
//             "autoTLS"=> auto_tls,
//             "mailer"=> mailer,
//             "fromName"=> from_name,
//             "fromEmail"=> from_email,
//             "replyToName"=> reply_to_name,
//             "replyToEmail"=> reply_to_email,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update SMTP provider
//     ///
//     /// Update a SMTP provider by its unique ID.
//     pub async fn update_smtp_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         host: Option<&str>,
//         port: Option<i32>,
//         username: Option<&str>,
//         password: Option<&str>,
//         encryption: Option<SmtpEncryption>,
//         auto_tls: Option<bool>,
//         mailer: Option<&str>,
//         from_name: Option<&str>,
//         from_email: Option<&str>,
//         reply_to_name: Option<&str>,
//         reply_to_email: Option<&str>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/smtp/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "host"=> host,
//             "port"=> port,
//             "username"=> username,
//             "password"=> password,
//             "encryption"=> encryption,
//             "autoTLS"=> auto_tls,
//             "mailer"=> mailer,
//             "fromName"=> from_name,
//             "fromEmail"=> from_email,
//             "replyToName"=> reply_to_name,
//             "replyToEmail"=> reply_to_email,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create Telesign provider
//     ///
//     /// Create a new Telesign provider.
//     pub async fn create_telesign_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         from: Option<&str>,
//         customer_id: Option<&str>,
//         api_key: Option<&str>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/telesign";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=> Some(name),
//             "from"=> from,
//             "customId"=> customer_id,
//             "apiKey"=> api_key,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update Telesign provider
//     ///
//     /// Update a Telesign provider by its unique ID.
//     pub async fn update_telesign_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         enabled: Option<bool>,
//         customer_id: Option<&str>,
//         api_key: Option<&str>,
//         from: Option<&str>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/telesign/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "from"=> from,
//             "customId"=> customer_id,
//             "apiKey"=> api_key,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create Textmagic provider
//     ///
//     /// Create a new Textmagic provider.
//     pub async fn create_text_magic_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         from: Option<&str>,
//         username: Option<&str>,
//         api_key: Option<&str>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/textmagic";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=> Some(name),
//             "from"=> from,
//             "username"=> username,
//             "apiKey"=> api_key,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update Textmagic provider
//     ///
//     /// Update a Textmagic provider by its unique ID.
//     pub async fn update_text_magic_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         enabled: Option<bool>,
//         username: Option<&str>,
//         api_key: Option<&str>,
//         from: Option<&str>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/textmagic/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "from"=> from,
//             "username"=> username,
//             "apiKey"=> api_key,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create Twilio provider
//     ///
//     /// Create a new Twilio provider.
//     pub async fn create_twilio_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         from: Option<&str>,
//         account_id: Option<&str>,
//         auth_token: Option<&str>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/twilio";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=> Some(name),
//             "from"=> from,
//             "accountSid"=> account_id,
//             "authToken"=> auth_token,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update Twilio provider
//     ///
//     /// Update a Twilio provider by its unique ID.
//     pub async fn update_twilio_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         enabled: Option<bool>,
//         accounts_id: Option<&str>,
//         auth_token: Option<&str>,
//         from: Option<&str>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/twilio/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "from"=> from,
//             "accountSid"=> accounts_id,
//             "authToken"=> auth_token,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create Vonage provider
//     ///
//     /// Create a new Vonage provider.
//     pub async fn create_vonage_provider(
//         client: &Client,
//         provider_id: &str,
//         name: &str,
//         from: Option<&str>,
//         api_key: Option<&str>,
//         api_secret: Option<&str>,
//         enabled: Option<bool>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/providers/vonage";

//         let api_params = api_params!(
//             "providerId"=> Some(provider_id),
//             "name"=> Some(name),
//             "from"=> from,
//             "apiKey"=> api_key,
//             "apiSecret"=> api_secret,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update Vonage provider
//     ///
//     /// Update a Vonage provider by its unique ID.
//     pub async fn update_vonage_provider(
//         client: &Client,
//         provider_id: &str,
//         name: Option<&str>,
//         enabled: Option<bool>,
//         api_key: Option<&str>,
//         api_secret: Option<&str>,
//         from: Option<&str>,
//     ) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/vonage/{}", provider_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "from"=> from,
//             "apiKey"=> api_key,
//             "apiSecret"=> api_secret,
//             "enabled"=> enabled,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get provider
//     ///
//     /// Get a provider by its unique ID.
//     ///
//     pub async fn get_provider(client: &Client, provider_id: &str) -> Result<Provider, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/{}", provider_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete provider
//     ///
//     /// Delete a provider by its unique ID.
//     pub async fn delete_provider(client: &Client, provider_id: &str) -> Result<(), Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/{}", provider_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }

//     /// List provider logs
//     ///
//     /// Get the provider activity logs listed by its unique ID.
//     pub async fn list_provider_logs(
//         client: &Client,
//         provider_id: &str,
//         queries: Option<Vec<String>>,
//     ) -> Result<LogList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/providers/{}/logs", provider_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List subscriber logs
//     ///
//     /// Get the subscriber activity logs listed by its unique ID.
//     pub async fn list_subscriber_logs(
//         client: &Client,
//         subscriber_id: &str,
//         queries: Option<Vec<String>>,
//     ) -> Result<LogList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/subscribers/{}/logs", subscriber_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List topics
//     ///
//     /// Get a list of all topics from the current Appwrite project.
//     pub async fn list_topics(
//         client: &Client,
//         queries: Option<Vec<String>>,
//         search: Option<String>,
//     ) -> Result<TopicList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/topics";

//         let api_params = api_params!(
//             "queries"=> queries,
//             "search"=> search,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create topic
//     ///
//     /// Create a new topic.
//     pub async fn create_topics(
//         client: &Client,
//         topic_id: &str,
//         name: &str,
//         subscribe: Option<Vec<&str>>,
//     ) -> Result<Topic, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/messaging/topics";

//         let api_params = api_params!(
//             "topicId"=> Some(topic_id),
//             "name"=> Some(name),
//             "subscribe"=> subscribe,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get topic
//     ///
//     /// Get a topic by its unique ID.
//     ///
//     pub async fn get_topic(client: &Client, topic_id: &str) -> Result<Topic, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/topics/{}", topic_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Update topic
//     ///
//     /// Update a topic by its unique ID.
//     ///
//     pub async fn update_topic(
//         client: &Client,
//         topic_id: &str,
//         name: Option<&str>,
//         subscribe: Option<Vec<&str>>,
//     ) -> Result<Topic, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/topics/{}", topic_id);

//         let api_params = api_params!(
//             "name"=> name,
//             "subscribe"=> subscribe,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::PATCH,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Delete topic
//     ///
//     /// Delete a topic by its unique ID.
//     pub async fn delete_topic(client: &Client, topic_id: &str) -> Result<(), Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/topics/{}", topic_id);

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }

//     /// List topic logs
//     ///
//     /// Get the topic activity logs listed by its unique ID.
//     pub async fn list_topic_logs(
//         client: &Client,
//         topic_id: &str,
//         queries: Option<Vec<String>>,
//     ) -> Result<LogList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/topics/{}/logs", topic_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// List subscribers
//     ///
//     /// Get a list of all subscribers from the current Appwrite project.
//     pub async fn list_subscribers(
//         client: &Client,
//         topic_id: &str,
//         queries: Option<Vec<String>>,
//         search: Option<String>,
//     ) -> Result<SubscriberList, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/topics/{}/subscribers", topic_id);

//         let api_params = api_params!(
//             "queries"=> queries,
//             "search"=> search,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Create subscriber
//     ///
//     /// Create a new subscriber.
//     pub async fn create_subscriber(
//         client: &Client,
//         topic_id: &str,
//         subscriber_id: &str,
//         target_id: &str,
//     ) -> Result<Subscriber, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/messaging/topics/{}/subscribers", topic_id);

//         let api_params = api_params!(
//             "subscriberId"=> Some(subscriber_id),
//             "targetId"=> Some(target_id),
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::POST,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get subscriber
//     ///
//     /// Get a subscriber by its unique ID.
//     ///
//     pub async fn get_subscriber(
//         client: &Client,
//         topic_id: &str,
//         subscriber_id: &str,
//     ) -> Result<Subscriber, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!(
//             "/messaging/topics/{}/subscribers/{}",
//             topic_id, subscriber_id
//         );

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(
//                 HttpMethod::GET,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(res.json().await?)
//     }

//     // Delete subscriber
//     ///
//     /// Delete a subscriber by its unique ID.
//     pub async fn delete_subscriber(
//         client: &Client,
//         topic_id: &str,
//         subscriber_id: &str,
//     ) -> Result<(), Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!(
//             "/messaging/topics/{}/subscribers/{}",
//             topic_id, subscriber_id
//         );

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let _res = client
//             .call(
//                 HttpMethod::DELETE,
//                 api_path.as_str(),
//                 api_headers,
//                 &api_params,
//                 None,
//             )
//             .await?;

//         Ok(())
//     }
// }
