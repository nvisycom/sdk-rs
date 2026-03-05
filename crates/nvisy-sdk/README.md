# nvisy-sdk

[![Crates.io](https://img.shields.io/crates/v/nvisy-sdk?style=flat-square)](https://crates.io/crates/nvisy-sdk)
[![Documentation](https://img.shields.io/docsrs/nvisy-sdk?style=flat-square)](https://docs.rs/nvisy-sdk)
[![Build](https://img.shields.io/github/actions/workflow/status/nvisycom/sdk-rs/build.yml?branch=main&label=build%20%26%20test&style=flat-square)](https://github.com/nvisycom/sdk-rs/actions/workflows/build.yml)

Rust client for the [Nvisy](https://nvisy.com) Server API.

The Nvisy Server provides authentication, workspace management, persistence,
and task routing for the Nvisy multimodal redaction platform. Use this crate
when connecting through the managed Nvisy service rather than directly to a
runtime instance.

## Installation

```toml
[dependencies]
nvisy-sdk = { version = "0.1", features = [] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust,no_run
use nvisy_sdk::{Nvisy, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Nvisy::with_api_key("your-api-key")?;
    // ...
    Ok(())
}
```

## Features

- `rustls-tls` *(default)*: use rustls for HTTPS
- `native-tls`: use platform-native TLS (mutually exclusive with `rustls-tls`)
- `tracing`: emit [tracing](https://docs.rs/tracing) spans and events for HTTP requests and client lifecycle

### Observability

Enable the `tracing` feature to instrument all HTTP requests and client operations:

```toml
nvisy-sdk = { version = "0.1", features = ["tracing"] }
```

The SDK emits spans and events under the `nvisy_sdk::client` target at the
following levels:

- **INFO**: client creation
- **DEBUG**: HTTP requests (method, URL, status, latency)
- **TRACE**: request construction details

## Getting Started

The fastest way to get started is with [Nvisy Cloud](https://nvisy.com).

To run locally, see the [runtime](https://github.com/nvisycom/runtime) and [server](https://github.com/nvisycom/server) repositories.

## License

MIT License, see [LICENSE.txt](../../LICENSE.txt)

## Support

- **Documentation**: [docs.nvisy.com](https://docs.nvisy.com)
- **API reference**: [docs.rs/nvisy-sdk](https://docs.rs/nvisy-sdk)
- **Issues**: [GitHub Issues](https://github.com/nvisycom/sdk-rs/issues)
- **Email**: [support@nvisy.com](mailto:support@nvisy.com)
