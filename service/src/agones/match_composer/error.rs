#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// HTTP connection failed (e.g. connection refused, DNS error)
    #[error("HTTP connection failed: {0}")]
    Connection(#[source] reqwest::Error),

    /// Server returned a non-2xx HTTP status
    #[error("HTTP request failed with status {status}: {body}")]
    RequestFailed { status: u16, body: String },

    /// Failed to deserialize the response body
    #[error("Failed to deserialize response: {0}")]
    DeserializeFailed(#[source] reqwest::Error),
}
