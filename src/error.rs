//! Error types for the `IDBuilder` SDK.

use std::fmt;

/// Result type alias using [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when using the `IDBuilder` SDK.
#[derive(Debug)]
pub enum Error {
    /// HTTP transport error.
    Http(HttpError),

    /// API returned an error response.
    Api {
        /// Error code from the API.
        code: i32,
        /// Error message from the API.
        message: String,
    },

    /// Key configuration not found.
    ConfigNotFound(String),

    /// Invalid or missing authentication token.
    Unauthorized,

    /// Token does not have permission for this operation.
    Forbidden,

    /// Rate limit exceeded.
    RateLimited,

    /// Sequence exhausted for the given key.
    SequenceExhausted(String),

    /// Invalid client configuration.
    InvalidConfig(String),

    /// Clock moved backwards (snowflake generation).
    ClockMovedBackwards,

    /// Sequence overflow (snowflake generation).
    SequenceOverflow,

    /// JSON serialization/deserialization error.
    Serialization(serde_json::Error),

    /// Invalid URL.
    InvalidUrl(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP error: {e}"),
            Self::Api { code, message } => write!(f, "API error (code={code}): {message}"),
            Self::ConfigNotFound(key) => write!(f, "Configuration not found: {key}"),
            Self::Unauthorized => write!(f, "Unauthorized: invalid or missing token"),
            Self::Forbidden => write!(f, "Forbidden: token not allowed for this operation"),
            Self::RateLimited => write!(f, "Rate limited"),
            Self::SequenceExhausted(key) => write!(f, "Sequence exhausted for key: {key}"),
            Self::InvalidConfig(msg) => write!(f, "Invalid configuration: {msg}"),
            Self::ClockMovedBackwards => write!(f, "Snowflake clock moved backwards"),
            Self::SequenceOverflow => write!(f, "Snowflake sequence overflow"),
            Self::Serialization(e) => write!(f, "Serialization error: {e}"),
            Self::InvalidUrl(url) => write!(f, "Invalid URL: {url}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Http(e) => Some(e),
            Self::Serialization(e) => Some(e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err)
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Self::Http(err)
    }
}

/// HTTP transport errors.
#[derive(Debug)]
pub enum HttpError {
    /// Connection failed.
    Connection(String),

    /// Request timed out.
    Timeout,

    /// Failed to read response body.
    ResponseBody(String),

    /// Other transport error.
    Other(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connection(msg) => write!(f, "Connection failed: {msg}"),
            Self::Timeout => write!(f, "Request timed out"),
            Self::ResponseBody(msg) => write!(f, "Failed to read response body: {msg}"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for HttpError {}
