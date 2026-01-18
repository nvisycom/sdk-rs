# Nvisy Rust SDK

Official Rust SDK for the Nvisy platform, providing a type-safe REST API client
with async/await support.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
nvisy-sdk = "0.1"
```

## Usage

```rust
use nvisy_sdk::{NvisyClient, NvisyConfig};
use nvisy_sdk::service::WorkspaceService;

#[tokio::main]
async fn main() -> nvisy_sdk::Result<()> {
    // Create a client with just an API key
    let client = NvisyClient::with_api_key("your-api-key")?;

    // Or use the builder for more options
    let client = NvisyConfig::builder()
        .with_api_key("your-api-key")
        .with_base_url("https://api.nvisy.com")
        .with_timeout_secs(60)
        .build_client()?;

    // Use the client
    let workspaces = client.list(None).await?;

    Ok(())
}
```

## Features

- Async/await support with Tokio
- Type-safe request and response models
- Configurable client with builder pattern
- Comprehensive error handling

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release notes and version history.

## License

MIT License - see [LICENSE.txt](LICENSE.txt)

## Support

- **Documentation**: [docs.nvisy.com](https://docs.nvisy.com)
- **Issues**: [GitHub Issues](https://github.com/nvisycom/sdk-rs/issues)
- **Email**: [support@nvisy.com](mailto:support@nvisy.com)
- **API Status**: [nvisy.openstatus.dev](https://nvisy.openstatus.dev)
