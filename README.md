# Nvisy SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/nvisy-sdk?style=flat-square)](https://crates.io/crates/nvisy-sdk)
[![Documentation](https://img.shields.io/docsrs/nvisy-sdk?style=flat-square)](https://docs.rs/nvisy-sdk)
[![Build](https://img.shields.io/github/actions/workflow/status/nvisycom/sdk-rs/build.yml?branch=main&label=build%20%26%20test&style=flat-square)](https://github.com/nvisycom/sdk-rs/actions/workflows/build.yml)

Rust client for the [Nvisy](https://nvisy.com/) multimodal redaction platform.

Nvisy detects and removes sensitive information across documents, images, and audio.
It combines deterministic patterns, NER, computer vision, and LLM-driven classification
into auditable, policy-driven pipelines built for regulated industries such as
healthcare, legal, government, and financial services.

> [!WARNING]
> **Active development: API not stable.** This project is under active
> development. Public APIs, configuration shapes, and on-disk formats may change
> without notice between releases. Pin a specific version if you depend on this
> in production.

## Installation

```toml
[dependencies]
nvisy-sdk = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust,no_run
use nvisy_sdk::service::MonitorService;
use nvisy_sdk::{Nvisy, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Nvisy::with_api_key("your-api-token")?;

    let health = client.health(None).await?;
    println!("Status: {:?}", health.status);

    Ok(())
}
```

The client can also be configured through the builder:

```rust,no_run
use std::time::Duration;

use nvisy_sdk::{Nvisy, Result};

# fn example() -> Result<()> {
let client = Nvisy::builder()
    .with_api_key("your-api-token") // Required
    .with_base_url("https://api.nvisy.com") // Optional
    .with_user_agent("MyApp/1.0.0") // Optional
    .with_timeout(Duration::from_secs(30)) // Optional
    .with_max_retries(3u32) // Optional
    .build()?;
# Ok(())
# }
```

See the [`examples/`](examples/) folder for more.

## Features

- `rustls-tls` *(default)*: use rustls for HTTPS
- `native-tls`: use platform-native TLS (mutually exclusive with `rustls-tls`)
- `tracing`: emit [tracing](https://docs.rs/tracing) spans and events for HTTP requests and client lifecycle

## Deployment

The fastest way to get started is with [Nvisy Cloud](https://nvisy.com).

To run locally, see the [nvisycom/runtime](https://github.com/nvisycom/runtime) and [nvisycom/server](https://github.com/nvisycom/server) repositories.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and contribution guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release notes and version history.

## License

MIT License, see [LICENSE.txt](LICENSE.txt)

## Support

- **Documentation**: [docs.nvisy.com](https://docs.nvisy.com)
- **API reference**: [docs.rs/nvisy-sdk](https://docs.rs/nvisy-sdk)
- **Issues**: [github.com/nvisycom/sdk-rs/issues](https://github.com/nvisycom/sdk-rs/issues)
- **Email**: [support@nvisy.com](mailto:support@nvisy.com)
