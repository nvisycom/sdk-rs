# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-03-05

### Added

- `nvisy-sdk`: async client for the Nvisy Server API (auth, workspaces, task routing)
- `nvisy-rt-sdk`: async client for the Nvisy Runtime API (direct redaction)
- Builder pattern for client construction with validation
- Configurable `base_url`, `timeout`, `max_retries`, and `user_agent`
- Automatic retries with exponential backoff via `reqwest-retry`
- Optional `tracing` feature for request/response observability
- TLS backend selection: `rustls-tls` (default) or `native-tls`
- Compile-time guards for mutually exclusive TLS features

[Unreleased]: https://github.com/nvisycom/sdk-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/nvisycom/sdk-rs/releases/tag/v0.1.0
