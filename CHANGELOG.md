# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `nvisy-rt-sdk`: optional `jsonschema` feature to derive `schemars::JsonSchema` for all models
- `nvisy-rt-sdk`: optional `mock` feature with `MockRuntime` for testing with configurable responses

### Changed

- `nvisy-rt-sdk`: rename `NvisyRt` to `Runtime`, `NvisyRtBuilder` to `RuntimeBuilder`

### Removed

- `nvisy-rt-sdk`: `Error::Reqwest` variant (reqwest errors now routed through `Error::Http`)

## [0.1.1] - 2026-03-22

### Added

- `nvisy-rt-sdk`: infra, runs, files, and contexts services aligned with server API
- `nvisy-rt-sdk`: `PageStream<T>` auto-paginating stream (`stream` feature)
- `nvisy-rt-sdk`: structured `ApiError`/`ErrorKind` error handling
- `nvisy-rt-sdk`: `base64` feature gate for file encoding helpers

### Changed

- `nvisy-rt-sdk`: `actor_id` moved from request models to client (`X-Actor-Id` header)
- `nvisy-rt-sdk`: `NvisyRt::new()` is now infallible, implements `Default`
- `nvisy-sdk`: health service aligned with server API (`ComponentCheck`, `timestamp`)

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

[Unreleased]: https://github.com/nvisycom/sdk-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/nvisycom/sdk-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/nvisycom/sdk-rs/releases/tag/v0.1.0
