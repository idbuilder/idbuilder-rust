//! `IDBuilder` Rust SDK
//!
//! A Rust client library for the `IDBuilder` distributed ID generation service.
//!
//! # Features
//!
//! - Support for all three ID types: auto-increment, snowflake, and formatted
//! - Both sync (default) and async (feature-gated) HTTP clients
//! - Local snowflake ID generation after fetching configuration
//! - Builder patterns for ergonomic API usage
//!
//! # Quick Start
//!
//! ```no_run
//! use idbuilder::{IdBuilderClient, Result};
//!
//! fn main() -> Result<()> {
//!     // Create client with key token
//!     let client = IdBuilderClient::new("http://localhost:8080", "my-key-token")?;
//!
//!     // Generate auto-increment IDs
//!     let ids = client.increment("order-id").generate(5)?;
//!     println!("Generated IDs: {:?}", ids);
//!
//!     Ok(())
//! }
//! ```
//!
//! # ID Generation
//!
//! ## Auto-increment IDs
//!
//! ```no_run
//! use idbuilder::{IdBuilderClient, Result};
//!
//! fn main() -> Result<()> {
//!     let client = IdBuilderClient::new("http://localhost:8080", "my-key-token")?;
//!
//!     // Generate multiple IDs
//!     let ids = client.increment("order-id").generate(5)?;
//!     // [1001, 1002, 1003, 1004, 1005]
//!
//!     // Generate a single ID
//!     let id = client.increment("order-id").generate_one()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Formatted IDs
//!
//! ```no_run
//! use idbuilder::{IdBuilderClient, Result};
//!
//! fn main() -> Result<()> {
//!     let client = IdBuilderClient::new("http://localhost:8080", "my-key-token")?;
//!
//!     let ids = client.formatted("invoice-id").generate(3)?;
//!     // ["INV20240115-0001", "INV20240115-0002", "INV20240115-0003"]
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Snowflake IDs (Local Generation)
//!
//! For snowflake IDs, the SDK fetches configuration once and generates IDs locally:
//!
//! ```no_run
//! use idbuilder::{IdBuilderClient, Result};
//!
//! fn main() -> Result<()> {
//!     let client = IdBuilderClient::new("http://localhost:8080", "my-key-token")?;
//!
//!     // Get snowflake config and create local generator
//!     let config = client.snowflake("user-id").get_config()?;
//!     let generator = config.into_generator();
//!
//!     // Generate IDs locally (no network calls)
//!     let id = generator.next_id()?;
//!     let batch = generator.next_ids(100)?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

mod client;
mod config;
mod error;
mod snowflake;

pub mod api;
pub mod http;
pub mod types;

pub use client::IdBuilderClient;
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use snowflake::SnowflakeGenerator;
pub use types::response::{ApiResponse, SnowflakeIdResponse};
