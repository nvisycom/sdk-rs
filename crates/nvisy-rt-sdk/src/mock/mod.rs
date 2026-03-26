//! Mock client for testing.
//!
//! Requires the `mock` feature. Provides [`MockRuntime`], a configurable
//! mock that implements all service traits with preset responses.
//!
//! # Example
//!
//! ```rust
//! use nvisy_rt_sdk::mock::MockRuntime;
//! use nvisy_rt_sdk::model::{Health, ServiceStatus};
//! use nvisy_rt_sdk::service::InfraService;
//! use jiff::Timestamp;
//!
//! # async fn example() -> nvisy_rt_sdk::Result<()> {
//! let mock = MockRuntime::new()
//!     .on_health(|_| Ok(Health {
//!         status: ServiceStatus::Healthy,
//!         checks: vec![],
//!         timestamp: Timestamp::now(),
//!     }));
//!
//! let health = mock.health().await?;
//! assert_eq!(health.status, ServiceStatus::Healthy);
//! # Ok(())
//! # }
//! ```

mod client;
mod service;

pub use client::MockRuntime;
