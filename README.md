# Nvisy SDK

[![Crates.io](https://img.shields.io/crates/v/nvisy-sdk?style=flat-square&color=black)](https://crates.io/crates/nvisy-sdk)
[![Documentation](https://img.shields.io/docsrs/nvisy-sdk?style=flat-square&color=black)](https://docs.rs/nvisy-sdk)
[![Build](https://img.shields.io/github/actions/workflow/status/nvisycom/sdk-rs/build.yml?style=flat-square&color=black)](https://github.com/nvisycom/sdk-rs/actions)

A Rust client library for the [Nvisy](https://nvisy.com/) platform. This SDK
provides a type-safe, ergonomic interface for managing workspaces, documents,
and other Nvisy resources.

## Features

- **Type Safety**: Strongly typed models with comprehensive validation
- **Async/Await**: Built on modern async Rust with `tokio` and `reqwest`
- **Builder Pattern**: Flexible client configuration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
nvisy-sdk = { version = "0.1", features = [] }
```

## Quick Start

### Builder Configuration

```rust,no_run
use nvisy_sdk::{NvisyConfig, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let client = NvisyConfig::builder()
        .with_api_key("your-api-key")
        .with_base_url("https://api.nvisy.com")
        .with_timeout(Duration::from_secs(60))
        .build_client()?;

    // Use the client for API calls...

    Ok(())
}
```

### Simple API Key

```rust,no_run
use nvisy_sdk::{NvisyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = NvisyClient::with_api_key("your-api-key")?;

    // Use the client for API calls...

    Ok(())
}
```

## Optional Features

### TLS Backend

Choose between two TLS implementations:

```toml
# Default: rustls-tls (recommended)
nvisy-sdk = { version = "0.1", features = [] }

# Alternative: native-tls
nvisy-sdk = { version = "0.1", features = ["native-tls"], default-features = false }
```

### Tracing Support

Enable comprehensive logging and tracing via the [`tracing`](https://crates.io/crates/tracing) crate:

```toml
nvisy-sdk = { version = "0.1", features = ["tracing"] }
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md)
for details on how to submit pull requests, report issues, and contribute to the
project.

## License

This project is licensed under the MIT License - see the
[LICENSE.txt](LICENSE.txt) file for details.

## Resources

- [Nvisy Documentation](https://docs.nvisy.com)
- [Full API Documentation](https://docs.rs/nvisy-sdk)
- [GitHub Issues](https://github.com/nvisycom/sdk-rs/issues)
