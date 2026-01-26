//! Client configuration.

use std::time::Duration;

/// Configuration for the `IDBuilder` client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL of the `IDBuilder` service.
    pub base_url: String,

    /// Key token for ID generation.
    pub key_token: Option<String>,

    /// Request timeout.
    pub timeout: Duration,

    /// Number of retries for failed requests.
    pub retries: u32,
}

impl ClientConfig {
    /// Default request timeout (30 seconds).
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

    /// Default number of retries.
    pub const DEFAULT_RETRIES: u32 = 0;

    /// Create a new configuration with the given base URL.
    #[must_use]
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            key_token: None,
            timeout: Self::DEFAULT_TIMEOUT,
            retries: Self::DEFAULT_RETRIES,
        }
    }

    /// Set the key token.
    #[must_use]
    pub fn with_key_token(mut self, token: impl Into<String>) -> Self {
        self.key_token = Some(token.into());
        self
    }

    /// Set the request timeout.
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the number of retries.
    #[must_use]
    pub const fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            key_token: None,
            timeout: Self::DEFAULT_TIMEOUT,
            retries: Self::DEFAULT_RETRIES,
        }
    }
}

/// Builder for [`ClientConfig`].
#[derive(Debug, Default)]
pub struct ClientConfigBuilder {
    base_url: Option<String>,
    key_token: Option<String>,
    timeout: Option<Duration>,
    retries: Option<u32>,
}

impl ClientConfigBuilder {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL.
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the key token.
    #[must_use]
    pub fn key_token(mut self, token: impl Into<String>) -> Self {
        self.key_token = Some(token.into());
        self
    }

    /// Set the request timeout.
    #[must_use]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the number of retries.
    #[must_use]
    pub const fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    /// Build the configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the base URL is not set.
    pub fn build(self) -> crate::Result<ClientConfig> {
        let base_url = self
            .base_url
            .ok_or_else(|| crate::Error::InvalidConfig("base_url is required".to_string()))?;

        Ok(ClientConfig {
            base_url,
            key_token: self.key_token,
            timeout: self.timeout.unwrap_or(ClientConfig::DEFAULT_TIMEOUT),
            retries: self.retries.unwrap_or(ClientConfig::DEFAULT_RETRIES),
        })
    }
}
