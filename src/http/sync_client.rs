//! Synchronous HTTP client using ureq.

use std::time::Duration;

use crate::error::HttpError;
use crate::http::{HttpClient, Response};
use crate::Result;

/// Synchronous HTTP client based on ureq.
#[derive(Debug)]
pub struct SyncHttpClient {
    agent: ureq::Agent,
}

impl SyncHttpClient {
    /// Create a new sync HTTP client with the given timeout.
    #[must_use]
    pub fn new(timeout: Duration) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(timeout)
            .timeout_read(timeout)
            .timeout_write(timeout)
            .build();
        Self { agent }
    }

    /// Create a new sync HTTP client with default timeout (30 seconds).
    #[must_use]
    pub fn with_default_timeout() -> Self {
        Self::new(Duration::from_secs(30))
    }
}

impl HttpClient for SyncHttpClient {
    fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<Response> {
        let mut req = self.agent.get(url);
        for (key, value) in headers {
            req = req.set(key, value);
        }

        match req.call() {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.into_string().map_err(|e| {
                    HttpError::ResponseBody(format!("Failed to read response body: {e}"))
                })?;
                Ok(Response::new(status, body))
            }
            Err(ureq::Error::Status(status, resp)) => {
                let body = resp.into_string().unwrap_or_default();
                Ok(Response::new(status, body))
            }
            Err(ureq::Error::Transport(e)) => Err(map_transport_error(&e).into()),
        }
    }

    fn post(&self, url: &str, headers: &[(&str, &str)], body: &str) -> Result<Response> {
        let mut req = self.agent.post(url);
        for (key, value) in headers {
            req = req.set(key, value);
        }
        req = req.set("Content-Type", "application/json");

        match req.send_string(body) {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.into_string().map_err(|e| {
                    HttpError::ResponseBody(format!("Failed to read response body: {e}"))
                })?;
                Ok(Response::new(status, body))
            }
            Err(ureq::Error::Status(status, resp)) => {
                let body = resp.into_string().unwrap_or_default();
                Ok(Response::new(status, body))
            }
            Err(ureq::Error::Transport(e)) => Err(map_transport_error(&e).into()),
        }
    }
}

fn map_transport_error(err: &ureq::Transport) -> HttpError {
    use ureq::ErrorKind;

    match err.kind() {
        ErrorKind::Io | ErrorKind::ConnectionFailed => HttpError::Connection(err.to_string()),
        ErrorKind::TooManyRedirects => HttpError::Other("Too many redirects".to_string()),
        ErrorKind::UnknownScheme => HttpError::Other(format!("Unknown URL scheme: {err}")),
        ErrorKind::Dns => HttpError::Connection(format!("DNS resolution failed: {err}")),
        ErrorKind::InsecureRequestHttpsOnly => {
            HttpError::Other("HTTPS required but HTTP was used".to_string())
        }
        ErrorKind::InvalidProxyUrl => HttpError::Other(format!("Invalid proxy URL: {err}")),
        ErrorKind::ProxyConnect => HttpError::Connection(format!("Proxy connection failed: {err}")),
        ErrorKind::ProxyUnauthorized => HttpError::Other("Proxy authentication failed".to_string()),
        _ => HttpError::Other(err.to_string()),
    }
}

impl Default for SyncHttpClient {
    fn default() -> Self {
        Self::with_default_timeout()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_client() {
        let client = SyncHttpClient::new(Duration::from_secs(10));
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_default_client() {
        let client = SyncHttpClient::default();
        assert!(std::mem::size_of_val(&client) > 0);
    }
}
