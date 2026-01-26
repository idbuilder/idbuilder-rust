//! Response types from API calls.

use serde::{Deserialize, Serialize};

use crate::SnowflakeGenerator;

/// Standard API response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Response code (0 = success, non-zero = error).
    pub code: i32,

    /// Human-readable message.
    pub message: String,

    /// Response data (null on error).
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// Check if the response indicates success.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.code == 0
    }

    /// Convert to Result, extracting data on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the response code is non-zero or data is missing.
    pub fn into_result(self) -> crate::Result<T> {
        if self.code == 0 {
            self.data.ok_or_else(|| crate::Error::Api {
                code: self.code,
                message: "Response data is missing".to_string(),
            })
        } else {
            Err(crate::Error::Api {
                code: self.code,
                message: self.message,
            })
        }
    }
}

/// Response for increment ID generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementIdResponse {
    /// List of generated IDs.
    pub ids: Vec<i64>,
}

/// Response for formatted ID generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedIdResponse {
    /// List of generated IDs.
    pub ids: Vec<String>,
}

/// Response for snowflake configuration request.
///
/// Contains all parameters needed for client-side ID generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnowflakeIdResponse {
    /// Allocated worker ID for this client.
    pub worker_id: u32,

    /// Custom epoch timestamp in milliseconds.
    pub epoch: i64,

    /// Number of bits for worker ID.
    pub worker_bits: u8,

    /// Number of bits for sequence number.
    pub sequence_bits: u8,
}

impl SnowflakeIdResponse {
    /// Convert this response into a local snowflake generator.
    #[must_use]
    pub const fn into_generator(self) -> SnowflakeGenerator {
        SnowflakeGenerator::new(
            self.worker_id,
            self.epoch,
            self.worker_bits,
            self.sequence_bits,
        )
    }
}
