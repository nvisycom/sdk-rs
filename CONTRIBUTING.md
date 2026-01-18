# Contributing

Thank you for your interest in contributing to the Nvisy Rust SDK.

## Requirements

- Rust 1.89+ (nightly for formatting)

## Setup

```bash
git clone https://github.com/nvisycom/sdk-rs.git
cd sdk-rs
cargo build
```

### SSH Access

Some dependencies are fetched from private GitHub repositories via SSH. Ensure
your SSH key is added to your GitHub account and ssh-agent is running:

```bash
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519
ssh -T git@github.com  # verify access
```

If cargo fails to fetch git dependencies, enable CLI-based git fetching:

```bash
export CARGO_NET_GIT_FETCH_WITH_CLI=true
```

## Development

Run all checks locally before submitting a pull request:

```bash
cargo check
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo doc --no-deps
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run all checks to verify they pass
5. Submit a pull request

## Security

- Never commit secrets or API keys
- Use environment variables for configuration
- Validate all external inputs

## License

By contributing, you agree your contributions will be licensed under the MIT
License.
