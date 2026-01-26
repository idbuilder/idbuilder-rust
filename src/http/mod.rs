//! HTTP client abstraction layer.

#[cfg(feature = "sync")]
mod sync_client;

#[cfg(feature = "sync")]
pub use sync_client::SyncHttpClient;

#[cfg(feature = "async")]
mod async_client;

#[cfg(feature = "async")]
pub use async_client::AsyncHttpClient;

/// HTTP response from the server.
#[derive(Debug)]
pub struct Response {
    /// HTTP status code.
    pub status: u16,
    /// Response body as string.
    pub body: String,
}

impl Response {
    /// Create a new response.
    #[must_use]
    pub const fn new(status: u16, body: String) -> Self {
        Self { status, body }
    }

    /// Check if the response indicates success (2xx).
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

/// Trait for HTTP client implementations.
pub trait HttpClient {
    /// Perform a GET request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    fn get(&self, url: &str, headers: &[(&str, &str)]) -> crate::Result<Response>;

    /// Perform a POST request with JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    fn post(&self, url: &str, headers: &[(&str, &str)], body: &str) -> crate::Result<Response>;
}
