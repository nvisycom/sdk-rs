# Contributing

Thank you for your interest in contributing to the Nvisy Rust SDK.

## Requirements

- Rust 1.92+

## Setup

```bash
git clone https://github.com/nvisycom/sdk-rs.git
cd sdk-rs
cargo build
```

## Development

Run all checks locally before submitting a pull request:

```bash
make check
```

This runs formatting, clippy, tests, and doc builds. See the [Makefile](Makefile) for individual targets.

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run all checks to verify they pass
5. Submit a pull request

## Security

- Never commit secrets or API keys.
- Use environment variables for configuration.

## License

By contributing, you agree your contributions will be licensed under the MIT
License.
