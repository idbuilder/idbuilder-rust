# idbuilder-rust

Rust SDK for the IDBuilder distributed ID generation service.

## Features

- Support for all three ID types: auto-increment, snowflake, and formatted
- Both sync (default) and async (feature-gated) HTTP clients
- Local snowflake ID generation after fetching configuration
- Minimal dependencies

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
idbuilder = "0.1"
```

For async support:

```toml
[dependencies]
idbuilder = { version = "0.1", features = ["async"] }
```

## Quick Start

```rust
use idbuilder::{IdBuilderClient, Result};

fn main() -> Result<()> {
    // Create client with key token
    let client = IdBuilderClient::new("http://localhost:8080", "my-key-token")?;

    // Generate auto-increment IDs
    let ids = client.increment("order-id").generate(5)?;
    println!("Generated IDs: {:?}", ids);

    Ok(())
}
```

## ID Generation

### Auto-increment IDs

```rust
let ids = client.increment("order-id").generate(5)?;
// [1001, 1002, 1003, 1004, 1005]

let single_id = client.increment("order-id").generate_one()?;
// 1006
```

### Formatted IDs

```rust
let ids = client.formatted("invoice-id").generate(3)?;
// ["INV20240115-0001", "INV20240115-0002", "INV20240115-0003"]
```

### Snowflake IDs (Local Generation)

For snowflake IDs, the SDK fetches configuration once and generates IDs locally:

```rust
// Get snowflake config and create local generator
let config = client.snowflake("user-id").get_config()?;
let generator = config.into_generator();

// Generate IDs locally (no network calls)
let id = generator.next_id()?;
let batch = generator.next_ids(100)?;

// Decompose an ID to inspect its parts
let (timestamp, worker_id, sequence) = generator.decompose(id);
```

The `SnowflakeGenerator` is thread-safe and can be shared across threads.

## Error Handling

```rust
use idbuilder::{Error, Result};

fn example() -> Result<()> {
    // ... operations ...

    Ok(())
}

fn main() {
    match example() {
        Ok(()) => println!("Success"),
        Err(Error::Unauthorized) => println!("Invalid token"),
        Err(Error::ConfigNotFound(key)) => println!("Config not found: {}", key),
        Err(Error::SequenceExhausted(key)) => println!("Sequence exhausted: {}", key),
        Err(Error::ClockMovedBackwards) => println!("System clock issue"),
        Err(e) => println!("Error: {}", e),
    }
}
```

## Features

| Feature | Description | Default |
|---------|-------------|---------|
| `sync` | Synchronous HTTP client using ureq | Yes |
| `async` | Async HTTP client using reqwest | No |
| `tls-rustls` | Use rustls for TLS | Yes |
| `tls-native` | Use native TLS | No |

## License

Apache-2.0
