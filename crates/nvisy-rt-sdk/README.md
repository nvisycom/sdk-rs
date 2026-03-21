# nvisy-rt-sdk

[![Crates.io](https://img.shields.io/crates/v/nvisy-rt-sdk?style=flat-square)](https://crates.io/crates/nvisy-rt-sdk)
[![Documentation](https://img.shields.io/docsrs/nvisy-rt-sdk?style=flat-square)](https://docs.rs/nvisy-rt-sdk)
[![Build](https://img.shields.io/github/actions/workflow/status/nvisycom/sdk-rs/build.yml?branch=main&label=build%20%26%20test&style=flat-square)](https://github.com/nvisycom/sdk-rs/actions/workflows/build.yml)

Rust client for the [Nvisy](https://nvisy.com) Runtime API.

The Nvisy Runtime is the core redaction engine that detects and removes
sensitive information across documents, images, and audio. It combines
deterministic patterns, NER, computer vision, and LLM-driven classification
into auditable, policy-driven pipelines. Use this crate to connect directly
to a runtime instance without going through the Nvisy Server.

## Installation

```toml
[dependencies]
nvisy-rt-sdk = { version = "0.1", features = [] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust,no_run
use nvisy_rt_sdk::{NvisyRt, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = NvisyRt::new();
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
nvisy-rt-sdk = { version = "0.1", features = ["tracing"] }
```

## Getting Started

The fastest way to get started is with [Nvisy Cloud](https://nvisy.com).

To run locally, see the [nvisycom/runtime](https://github.com/nvisycom/runtime) and [nvisycom/server](https://github.com/nvisycom/server) repositories.

## License

MIT License, see [LICENSE.txt](../../LICENSE.txt)

## Support

- **Documentation**: [docs.nvisy.com](https://docs.nvisy.com)
- **API reference**: [docs.rs/nvisy-rt-sdk](https://docs.rs/nvisy-rt-sdk)
- **Issues**: [GitHub Issues](https://github.com/nvisycom/sdk-rs/issues)
- **Email**: [support@nvisy.com](mailto:support@nvisy.com)
