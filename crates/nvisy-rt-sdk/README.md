# nvisy-rt-sdk

[![Crates.io](https://img.shields.io/crates/v/nvisy-rt-sdk?style=flat-square&color=black)](https://crates.io/crates/nvisy-rt-sdk)
[![Documentation](https://img.shields.io/docsrs/nvisy-rt-sdk?style=flat-square&color=black)](https://docs.rs/nvisy-rt-sdk)

Rust client library for the Nvisy Runtime API (direct task handling).

## Installation

```toml
[dependencies]
nvisy-rt-sdk = "0.1"
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

## Documentation

- [docs.nvisy.com](https://docs.nvisy.com)
- [API reference on docs.rs](https://docs.rs/nvisy-rt-sdk)

## License

MIT License, see [LICENSE.txt](../../LICENSE.txt)

## Support

- **Documentation**: [docs.nvisy.com](https://docs.nvisy.com)
- **Issues**: [GitHub Issues](https://github.com/nvisycom/sdk-rs/issues)
- **Email**: [support@nvisy.com](mailto:support@nvisy.com)
