use serde::{Deserialize, Serialize};

use super::headers::Headers;

/// Execution
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Execution {
    /// Execution ID.
    #[serde(rename = "$id")]
    pub id: String,

    /// Execution creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,

    /// Execution upate date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,

    /// Execution roles.
    #[serde(rename = "$permissions")]
    pub permissions: Vec<String>,

    /// Function ID.
    #[serde(rename = "functionId")]
    pub function_id: String,

    /// The trigger that caused the function to execute. Possible values can be: `http`, `schedule`, or `event`.
    pub trigger: String,

    /// The status of the function execution. Possible values can be: `waiting`, `processing`, `completed`, or `failed`.
    pub status: String,

    /// HTTP request method type.
    #[serde(rename = "requestMethod")]
    pub request_method: String,

    /// HTTP request path and query.
    #[serde(rename = "requestPath")]
    pub request_path: String,

    /// HTTP response headers as a key-value object. This will return only whitelisted headers. All headers are returned if execution is created as synchronous.
    #[serde(rename = "requestHeaders")]
    pub request_headers: Vec<Headers>,

    /// HTTP response status code.
    #[serde(rename = "responseStatusCode")]
    pub response_status_code: usize,

    /// HTTP response body. This will return empty unless execution is created as synchronous.
    #[serde(rename = "responseBody")]
    pub response_body: String,

    /// HTTP response headers as a key-value object. This will return only whitelisted headers. All headers are returned if execution is created as synchronous.
    #[serde(rename = "responseHeaders")]
    pub response_headers: Vec<Headers>,

    /// Function logs. Includes the last 4,000 characters. This will return an empty string unless the response is returned using an API key or as part of a webhook payload.
    pub logs: String,

    /// Function errors. Includes the last 4,000 characters. This will return an empty string unless the response is returned using an API key or as part of a webhook payload.
    pub errors: String,

    /// Function execution duration in seconds.
    pub duration: f64,

    /// The scheduled time for execution. If left empty, execution will be queued immediately.

    #[serde(rename = "scheduleAt")]
    pub schedule_at: Option<String>,
}
