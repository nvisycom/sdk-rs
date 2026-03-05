# nvisy-sdk

[![Crates.io](https://img.shields.io/crates/v/nvisy-sdk?style=flat-square&color=black)](https://crates.io/crates/nvisy-sdk)
[![Documentation](https://img.shields.io/docsrs/nvisy-sdk?style=flat-square&color=black)](https://docs.rs/nvisy-sdk)

Rust client library for the Nvisy managed API (auth, persistence, task redirection).

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
