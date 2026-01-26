//! ID generation APIs.

use crate::http::HttpClient;
use crate::types::response::{
    ApiResponse, FormattedIdResponse, IncrementIdResponse, SnowflakeIdResponse,
};
use crate::{Error, Result};

/// Auto-increment ID generation API.
#[derive(Debug)]
pub struct IncrementApi<'a, C: HttpClient> {
    base_url: &'a str,
    key_token: &'a str,
    client: &'a C,
    key: String,
}

impl<'a, C: HttpClient> IncrementApi<'a, C> {
    /// Create a new increment API instance.
    pub(crate) fn new(
        base_url: &'a str,
        key_token: &'a str,
        client: &'a C,
        key: impl Into<String>,
    ) -> Self {
        Self {
            base_url,
            key_token,
            client,
            key: key.into(),
        }
    }

    /// Generate a single auto-increment ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the sequence is exhausted.
    pub fn generate_one(&self) -> Result<i64> {
        let ids = self.generate(1)?;
        ids.into_iter().next().ok_or_else(|| Error::Api {
            code: 0,
            message: "No IDs returned".to_string(),
        })
    }

    /// Generate multiple auto-increment IDs.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of IDs to generate (max 1000)
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the sequence is exhausted.
    pub fn generate(&self, count: u32) -> Result<Vec<i64>> {
        let url = format!(
            "{}/v1/id/increment?key={}&size={}",
            self.base_url,
            urlencoding::encode(&self.key),
            count
        );
        let headers = [("Authorization", self.key_token)];

        let response = self.client.get(&url, &headers)?;

        match response.status {
            200 => {
                let api_resp: ApiResponse<IncrementIdResponse> =
                    serde_json::from_str(&response.body)?;
                Ok(api_resp.into_result()?.ids)
            }
            401 => Err(Error::Unauthorized),
            403 => Err(Error::Forbidden),
            404 => Err(Error::ConfigNotFound(self.key.clone())),
            429 => Err(Error::RateLimited),
            _ => {
                let api_resp: ApiResponse<()> = serde_json::from_str(&response.body)
                    .unwrap_or_else(|_| ApiResponse {
                        code: response.status.into(),
                        message: response.body.clone(),
                        data: None,
                    });

                // Check for sequence exhausted error
                if api_resp.message.to_lowercase().contains("exhausted") {
                    return Err(Error::SequenceExhausted(self.key.clone()));
                }

                Err(Error::Api {
                    code: api_resp.code,
                    message: api_resp.message,
                })
            }
        }
    }
}

/// Snowflake ID generation API.
#[derive(Debug)]
pub struct SnowflakeApi<'a, C: HttpClient> {
    base_url: &'a str,
    key_token: &'a str,
    client: &'a C,
    key: String,
}

impl<'a, C: HttpClient> SnowflakeApi<'a, C> {
    /// Create a new snowflake API instance.
    pub(crate) fn new(
        base_url: &'a str,
        key_token: &'a str,
        client: &'a C,
        key: impl Into<String>,
    ) -> Self {
        Self {
            base_url,
            key_token,
            client,
            key: key.into(),
        }
    }

    /// Get the snowflake configuration for local ID generation.
    ///
    /// The returned configuration contains a worker ID assigned by the server
    /// and can be converted into a [`SnowflakeGenerator`](crate::SnowflakeGenerator)
    /// for local ID generation.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the configuration doesn't exist.
    pub fn get_config(&self) -> Result<SnowflakeIdResponse> {
        let url = format!(
            "{}/v1/id/snowflake?key={}",
            self.base_url,
            urlencoding::encode(&self.key)
        );
        let headers = [("Authorization", self.key_token)];

        let response = self.client.get(&url, &headers)?;

        match response.status {
            200 => {
                let api_resp: ApiResponse<SnowflakeIdResponse> =
                    serde_json::from_str(&response.body)?;
                api_resp.into_result()
            }
            401 => Err(Error::Unauthorized),
            403 => Err(Error::Forbidden),
            404 => Err(Error::ConfigNotFound(self.key.clone())),
            _ => {
                let api_resp: ApiResponse<()> = serde_json::from_str(&response.body)
                    .unwrap_or_else(|_| ApiResponse {
                        code: response.status.into(),
                        message: response.body.clone(),
                        data: None,
                    });
                Err(Error::Api {
                    code: api_resp.code,
                    message: api_resp.message,
                })
            }
        }
    }
}

/// Formatted string ID generation API.
#[derive(Debug)]
pub struct FormattedApi<'a, C: HttpClient> {
    base_url: &'a str,
    key_token: &'a str,
    client: &'a C,
    key: String,
}

impl<'a, C: HttpClient> FormattedApi<'a, C> {
    /// Create a new formatted API instance.
    pub(crate) fn new(
        base_url: &'a str,
        key_token: &'a str,
        client: &'a C,
        key: impl Into<String>,
    ) -> Self {
        Self {
            base_url,
            key_token,
            client,
            key: key.into(),
        }
    }

    /// Generate a single formatted ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the sequence is exhausted.
    pub fn generate_one(&self) -> Result<String> {
        let ids = self.generate(1)?;
        ids.into_iter().next().ok_or_else(|| Error::Api {
            code: 0,
            message: "No IDs returned".to_string(),
        })
    }

    /// Generate multiple formatted IDs.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of IDs to generate (max 1000)
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the sequence is exhausted.
    pub fn generate(&self, count: u32) -> Result<Vec<String>> {
        let url = format!(
            "{}/v1/id/formatted?key={}&size={}",
            self.base_url,
            urlencoding::encode(&self.key),
            count
        );
        let headers = [("Authorization", self.key_token)];

        let response = self.client.get(&url, &headers)?;

        match response.status {
            200 => {
                let api_resp: ApiResponse<FormattedIdResponse> =
                    serde_json::from_str(&response.body)?;
                Ok(api_resp.into_result()?.ids)
            }
            401 => Err(Error::Unauthorized),
            403 => Err(Error::Forbidden),
            404 => Err(Error::ConfigNotFound(self.key.clone())),
            429 => Err(Error::RateLimited),
            _ => {
                let api_resp: ApiResponse<()> = serde_json::from_str(&response.body)
                    .unwrap_or_else(|_| ApiResponse {
                        code: response.status.into(),
                        message: response.body.clone(),
                        data: None,
                    });

                // Check for sequence exhausted error
                if api_resp.message.to_lowercase().contains("exhausted") {
                    return Err(Error::SequenceExhausted(self.key.clone()));
                }

                Err(Error::Api {
                    code: api_resp.code,
                    message: api_resp.message,
                })
            }
        }
    }
}

/// URL encoding helper.
mod urlencoding {
    /// Percent-encode a string for use in URL query parameters.
    pub fn encode(input: &str) -> String {
        let mut result = String::with_capacity(input.len() * 3);
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                _ => {
                    result.push('%');
                    result.push(
                        char::from_digit(u32::from(byte >> 4), 16)
                            .unwrap()
                            .to_ascii_uppercase(),
                    );
                    result.push(
                        char::from_digit(u32::from(byte & 0xF), 16)
                            .unwrap()
                            .to_ascii_uppercase(),
                    );
                }
            }
        }
        result
    }
}
