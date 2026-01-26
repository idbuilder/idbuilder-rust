//! Asynchronous HTTP client using reqwest.

use std::time::Duration;

use crate::error::HttpError;
use crate::http::Response;
use crate::Result;

/// Asynchronous HTTP client based on reqwest.
#[derive(Debug, Clone)]
pub struct AsyncHttpClient {
    client: reqwest::Client,
}

impl AsyncHttpClient {
    /// Create a new async HTTP client with the given timeout.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be created.
    pub fn new(timeout: Duration) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| HttpError::Other(format!("Failed to create HTTP client: {e}")))?;
        Ok(Self { client })
    }

    /// Create a new async HTTP client with default timeout (30 seconds).
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be created.
    pub fn with_default_timeout() -> Result<Self> {
        Self::new(Duration::from_secs(30))
    }

    /// Perform an async GET request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<Response> {
        let mut req = self.client.get(url);
        for (key, value) in headers {
            req = req.header(*key, *value);
        }

        let resp = req.send().await.map_err(map_reqwest_error)?;
        let status = resp.status().as_u16();
        let body = resp
            .text()
            .await
            .map_err(|e| HttpError::ResponseBody(format!("Failed to read response body: {e}")))?;

        Ok(Response::new(status, body))
    }

    /// Perform an async POST request with JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn post(&self, url: &str, headers: &[(&str, &str)], body: &str) -> Result<Response> {
        let mut req = self.client.post(url);
        for (key, value) in headers {
            req = req.header(*key, *value);
        }
        req = req.header("Content-Type", "application/json");
        req = req.body(body.to_string());

        let resp = req.send().await.map_err(map_reqwest_error)?;
        let status = resp.status().as_u16();
        let body = resp
            .text()
            .await
            .map_err(|e| HttpError::ResponseBody(format!("Failed to read response body: {e}")))?;

        Ok(Response::new(status, body))
    }
}

fn map_reqwest_error(err: reqwest::Error) -> HttpError {
    if err.is_timeout() {
        HttpError::Timeout
    } else if err.is_connect() {
        HttpError::Connection(err.to_string())
    } else {
        HttpError::Other(err.to_string())
    }
}

impl Default for AsyncHttpClient {
    fn default() -> Self {
        Self::with_default_timeout().expect("Failed to create default HTTP client")
    }
}
