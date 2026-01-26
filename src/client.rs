//! Main client implementation.

use std::time::Duration;

use crate::api::{FormattedApi, IncrementApi, SnowflakeApi};
use crate::config::{ClientConfig, ClientConfigBuilder};
use crate::http::HttpClient;
use crate::Result;

#[cfg(feature = "sync")]
use crate::http::SyncHttpClient;

/// Client for the `IDBuilder` ID generation service.
///
/// This client uses a key token for ID generation operations.
///
/// # Example
///
/// ```no_run
/// use idbuilder::{IdBuilderClient, Result};
///
/// fn main() -> Result<()> {
///     // Create client with key token
///     let client = IdBuilderClient::new("http://localhost:8080", "my-key-token")?;
///
///     // Generate auto-increment IDs
///     let ids = client.increment("order-id").generate(5)?;
///     println!("Generated IDs: {:?}", ids);
///
///     // Generate formatted IDs
///     let ids = client.formatted("invoice-id").generate(3)?;
///     println!("Formatted IDs: {:?}", ids);
///
///     // Get snowflake config for local generation
///     let config = client.snowflake("user-id").get_config()?;
///     let generator = config.into_generator();
///     let id = generator.next_id()?;
///     println!("Snowflake ID: {}", id);
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct IdBuilderClient<C: HttpClient> {
    config: ClientConfig,
    http_client: C,
}

#[cfg(feature = "sync")]
impl IdBuilderClient<SyncHttpClient> {
    /// Create a new client with the given base URL and key token.
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL of the `IDBuilder` service
    /// * `key_token` - Key token for ID generation
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid.
    pub fn new(base_url: impl Into<String>, key_token: impl Into<String>) -> Result<Self> {
        let config = ClientConfig::new(base_url).with_key_token(key_token);
        let http_client = SyncHttpClient::new(config.timeout);
        Ok(Self {
            config,
            http_client,
        })
    }

    /// Create a new client builder.
    #[must_use]
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::new()
    }

    /// Create a client from a configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn from_config(config: ClientConfig) -> Result<Self> {
        let http_client = SyncHttpClient::new(config.timeout);
        Ok(Self {
            config,
            http_client,
        })
    }
}

impl<C: HttpClient> IdBuilderClient<C> {
    /// Create a new client with a custom HTTP client.
    #[must_use]
    pub const fn with_http_client(config: ClientConfig, http_client: C) -> Self {
        Self {
            config,
            http_client,
        }
    }

    /// Get the base URL.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// Get the request timeout.
    #[must_use]
    pub const fn timeout(&self) -> Duration {
        self.config.timeout
    }

    /// Access the auto-increment ID generation API for a specific key.
    ///
    /// # Arguments
    ///
    /// * `key` - The configuration key to generate IDs for
    ///
    /// # Panics
    ///
    /// Panics if no key token is configured.
    pub fn increment(&self, key: impl Into<String>) -> IncrementApi<'_, C> {
        let key_token = self
            .config
            .key_token
            .as_deref()
            .expect("Key token is required for ID generation");
        IncrementApi::new(&self.config.base_url, key_token, &self.http_client, key)
    }

    /// Access the snowflake ID generation API for a specific key.
    ///
    /// # Arguments
    ///
    /// * `key` - The configuration key to get snowflake config for
    ///
    /// # Panics
    ///
    /// Panics if no key token is configured.
    pub fn snowflake(&self, key: impl Into<String>) -> SnowflakeApi<'_, C> {
        let key_token = self
            .config
            .key_token
            .as_deref()
            .expect("Key token is required for ID generation");
        SnowflakeApi::new(&self.config.base_url, key_token, &self.http_client, key)
    }

    /// Access the formatted ID generation API for a specific key.
    ///
    /// # Arguments
    ///
    /// * `key` - The configuration key to generate IDs for
    ///
    /// # Panics
    ///
    /// Panics if no key token is configured.
    pub fn formatted(&self, key: impl Into<String>) -> FormattedApi<'_, C> {
        let key_token = self
            .config
            .key_token
            .as_deref()
            .expect("Key token is required for ID generation");
        FormattedApi::new(&self.config.base_url, key_token, &self.http_client, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::Response;

    struct MockHttpClient;

    impl HttpClient for MockHttpClient {
        fn get(&self, _url: &str, _headers: &[(&str, &str)]) -> Result<Response> {
            Ok(Response::new(
                200,
                r#"{"code":0,"message":"success","data":{"ids":[1,2,3]}}"#.to_string(),
            ))
        }

        fn post(&self, _url: &str, _headers: &[(&str, &str)], _body: &str) -> Result<Response> {
            Ok(Response::new(
                200,
                r#"{"code":0,"message":"success","data":null}"#.to_string(),
            ))
        }
    }

    #[test]
    fn test_client_with_mock() {
        let config = ClientConfig::new("http://localhost:8080").with_key_token("test-token");
        let client = IdBuilderClient::with_http_client(config, MockHttpClient);

        assert_eq!(client.base_url(), "http://localhost:8080");
    }

    #[test]
    fn test_api_access() {
        let config = ClientConfig::new("http://localhost:8080").with_key_token("test-token");
        let client = IdBuilderClient::with_http_client(config, MockHttpClient);

        let _increment_api = client.increment("test-key");
        let _snowflake_api = client.snowflake("test-key");
        let _formatted_api = client.formatted("test-key");
    }
}
