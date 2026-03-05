# nvisy-rt-sdk

[![Crates.io](https://img.shields.io/crates/v/nvisy-rt-sdk?style=flat-square&color=black)](https://crates.io/crates/nvisy-rt-sdk)
[![Documentation](https://img.shields.io/docsrs/nvisy-rt-sdk?style=flat-square&color=black)](https://docs.rs/nvisy-rt-sdk)

Rust client library for the Nvisy Runtime API (direct task handling).

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
    let client = NvisyRt::with_api_key("your-api-key")?;
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
- **API reference**: [docs.rs/nvisy-rt-sdk](https://docs.rs/nvisy-rt-sdk)
- **Issues**: [GitHub Issues](https://github.com/nvisycom/sdk-rs/issues)
- **Email**: [support@nvisy.com](mailto:support@nvisy.com)
