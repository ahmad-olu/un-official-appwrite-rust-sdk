// //! # Health
// //!
// //! The Health service allows you to both validate and monitor your Appwrite
// //! server&#039;s health.

// use serde_json::json;

// use crate::{
//     api_params, app_json_header,
//     client::Client,
//     enumm::HttpMethod,
//     enums::name::Name,
//     error::Error,
//     models::{
//         health_antivirus::HealthAntivirus, health_certificate::HealthCertificate,
//         health_queue::HealthQueue, health_status::HealthStatus, health_time::HealthTime,
//     },
// };

// pub struct Health;

// impl Health {
//     /// Get HTTP
//     ///
//     /// Check the Appwrite HTTP server is up and responsive.
//     pub async fn get(client: &Client) -> Result<HealthStatus, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get antivirus
//     ///
//     /// Check the Appwrite Antivirus server is up and connection is successful.
//     pub async fn get_antivirus(client: &Client) -> Result<HealthAntivirus, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/anti-virus";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get cache
//     ///
//     /// Check the Appwrite in-memory cache servers are up and connection is
//     /// successful.
//     pub async fn get_cache(client: &Client) -> Result<HealthStatus, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/cache";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get the SSL certificate for a domain
//     ///
//     /// Get the SSL certificate for a domain
//     pub async fn get_certificate(
//         client: &Client,
//         domain: Option<&str>,
//     ) -> Result<HealthCertificate, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/certificate";

//         let api_params = api_params!(
//             "domain"=> domain,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get DB
//     ///
//     /// Check the Appwrite database servers are up and connection is successful.
//     pub async fn get_db(client: &Client) -> Result<HealthStatus, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/db";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get pubsub
//     ///
//     /// Check the Appwrite pub-sub servers are up and connection is successful.
//     pub async fn get_pub_sub(client: &Client) -> Result<HealthStatus, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/pubsub";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get queue
//     ///
//     /// Check the Appwrite queue messaging servers are up and connection is
//     /// successful.
//     pub async fn get_queue(client: &Client) -> Result<HealthStatus, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get builds queue
//     ///
//     /// Get the number of builds that are waiting to be processed in the Appwrite
//     /// internal queue server.
//     pub async fn get_queue_builds(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/builds";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get certificates queue
//     ///
//     /// Get the number of certificates that are waiting to be issued against
//     /// [Letsencrypt](https://letsencrypt.org/) in the Appwrite internal queue
//     /// server.
//     pub async fn get_queue_certificates(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/certificates";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get databases queue
//     ///
//     /// Get the number of database changes that are waiting to be processed in the
//     /// Appwrite internal queue server.
//     pub async fn get_queue_databases(
//         client: &Client,
//         name: Option<&str>,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/databases";

//         let api_params = api_params!(
//             "name"=>name,
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get deletes queue
//     ///
//     /// Get the number of background destructive changes that are waiting to be
//     /// processed in the Appwrite internal queue server.
//     pub async fn get_queue_deletes(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/deletes";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get number of failed queue jobs
//     ///
//     /// Returns the amount of failed jobs in a given queue.
//     ///
//     pub async fn get_failed_jobs(
//         client: &Client,
//         name: Name,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = format!("/health/queue/failed/{}", json!(name));

//         let api_params = api_params!(
//             "threshold"=> threshold,
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

//     /// Get functions queue
//     ///
//     /// Get the number of function executions that are waiting to be processed in
//     /// the Appwrite internal queue server.
//     pub async fn get_queue_functions(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/functions";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get logs queue
//     ///
//     /// Get the number of logs that are waiting to be processed in the Appwrite
//     /// internal queue server.
//     pub async fn get_queue_logs(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/logs";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get mails queue
//     ///
//     /// Get the number of mails that are waiting to be processed in the Appwrite
//     /// internal queue server.
//     pub async fn get_queue_mails(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/mails";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get messaging queue
//     ///
//     /// Get the number of messages that are waiting to be processed in the Appwrite
//     /// internal queue server.
//     pub async fn get_queue_messaging(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/messaging";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get migrations queue
//     ///
//     /// Get the number of migrations that are waiting to be processed in the
//     /// Appwrite internal queue server.
//     pub async fn get_queue_migrations(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/migrations";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get usage queue
//     ///
//     /// Get the number of metrics that are waiting to be processed in the Appwrite
//     /// internal queue server.
//     pub async fn get_queue_usage(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/usage";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get usage dump queue
//     ///
//     /// Get the number of projects containing metrics that are waiting to be
//     /// processed in the Appwrite internal queue server.
//     pub async fn get_queue_usage_dump(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/usage-dump";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get webhooks queue
//     ///
//     /// Get the number of webhooks that are waiting to be processed in the Appwrite
//     /// internal queue server.
//     pub async fn get_queue_webhooks(
//         client: &Client,
//         threshold: Option<usize>,
//     ) -> Result<HealthQueue, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/queue/webhooks";

//         let api_params = api_params!(
//             "threshold"=> threshold,
//         );

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get storage
//     ///
//     /// Check the Appwrite storage device is up and connection is successful.
//     pub async fn get_storage(client: &Client) -> Result<HealthStatus, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/storage";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get local storage
//     ///
//     /// Check the Appwrite local storage device is up and connection is successful.
//     pub async fn get_storage_local(client: &Client) -> Result<HealthStatus, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/storage/local";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }

//     /// Get time
//     ///
//     /// Check the Appwrite server time is synced with Google remote NTP server. We
//     /// use this technology to smoothly handle leap seconds with no disruptive
//     /// events. The [Network Time
//     /// Protocol](https://en.wikipedia.org/wiki/Network_Time_Protocol) (NTP) is
//     /// used by hundreds of millions of computers and devices to synchronize their
//     /// clocks over the Internet. If your computer sets its own clock, it likely
//     /// uses NTP.
//     pub async fn get_time(client: &Client) -> Result<HealthTime, Error> {
//         //const API_PATH: &str = "/functions";
//         let api_path = "/health/time";

//         let api_params = api_params!();

//         let api_headers = app_json_header!();

//         let res = client
//             .call(HttpMethod::GET, api_path, api_headers, &api_params, None)
//             .await?;

//         Ok(res.json().await?)
//     }
// }
