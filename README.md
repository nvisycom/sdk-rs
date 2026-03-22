# Nvisy SDK for Rust

[![Build](https://img.shields.io/github/actions/workflow/status/nvisycom/sdk-rs/build.yml?branch=main&label=build%20%26%20test&style=flat-square)](https://github.com/nvisycom/sdk-rs/actions/workflows/build.yml)

Rust client libraries for the [Nvisy](https://nvisy.com/) multimodal redaction platform.

Nvisy detects and removes sensitive information across documents, images, and audio.
It combines deterministic patterns, NER, computer vision, and LLM-driven classification
into auditable, policy-driven pipelines built for regulated industries such as
healthcare, legal, government, and financial services.

## Crates

- [`nvisy-sdk`](crates/nvisy-sdk/) [![Crates.io](https://img.shields.io/crates/v/nvisy-sdk?style=flat-square)](https://crates.io/crates/nvisy-sdk) [![Docs](https://img.shields.io/docsrs/nvisy-sdk?style=flat-square)](https://docs.rs/nvisy-sdk): client for the Nvisy Server API (authentication, workspace management, persistence, and task routing)
- [`nvisy-rt-sdk`](crates/nvisy-rt-sdk/) [![Crates.io](https://img.shields.io/crates/v/nvisy-rt-sdk?style=flat-square)](https://crates.io/crates/nvisy-rt-sdk) [![Docs](https://img.shields.io/docsrs/nvisy-rt-sdk?style=flat-square)](https://docs.rs/nvisy-rt-sdk): client for the Nvisy Runtime API (direct redaction task execution)

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
- **Issues**: [github.com/nvisycom/sdk-rs/issues](https://github.com/nvisycom/sdk-rs/issues)
- **Email**: [support@nvisy.com](mailto:support@nvisy.com)
