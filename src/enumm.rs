/// HTTP methods.
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

/// Response Types
pub enum ResponseType {
    /// Transform the response data to JSON object only when the
    /// content-type of response is "application/json" .
    JSON,

    /// Transform the response data to a String encoded with UTF8.
    PLAIN,

    /// Get original bytes, the type of response will be Vec<u8>
    BYTES,
}
