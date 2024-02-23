use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::headers::Headers;

/// Execution
#[derive(Debug, Serialize, Deserialize)]
pub struct Execution {
    /// Execution ID.
    #[serde(rename = "$id")]
    id: String,

    /// Execution creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    created_at: String,

    /// Execution upate date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    updated_at: String,

    /// Execution roles.
    #[serde(rename = "$permissions")]
    permissions: Vec<String>,

    /// Function ID.
    #[serde(rename = "functionId")]
    function_id: String,

    /// The trigger that caused the function to execute. Possible values can be: `http`, `schedule`, or `event`.
    trigger: String,

    /// The status of the function execution. Possible values can be: `waiting`, `processing`, `completed`, or `failed`.
    status: String,

    /// HTTP request method type.
    #[serde(rename = "requestMethod")]
    request_method: String,

    /// HTTP request path and query.
    #[serde(rename = "requestPath")]
    request_path: String,

    /// HTTP response headers as a key-value object. This will return only whitelisted headers. All headers are returned if execution is created as synchronous.
    #[serde(rename = "requestHeaders")]
    request_headers: Vec<Headers>,

    /// HTTP response status code.
    #[serde(rename = "responseStatusCode")]
    response_status_code: u64,

    /// HTTP response body. This will return empty unless execution is created as synchronous.
    #[serde(rename = "responseBody")]
    response_body: String,

    /// HTTP response headers as a key-value object. This will return only whitelisted headers. All headers are returned if execution is created as synchronous.
    #[serde(rename = "responseHeaders")]
    response_headers: Vec<Headers>,

    /// Function logs. Includes the last 4,000 characters. This will return an empty string unless the response is returned using an API key or as part of a webhook payload.
    logs: String,

    /// Function errors. Includes the last 4,000 characters. This will return an empty string unless the response is returned using an API key or as part of a webhook payload.
    errors: String,

    /// Function execution duration in seconds.
    duration: f64,
}
