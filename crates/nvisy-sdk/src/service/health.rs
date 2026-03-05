//! Health API service.
//!
//! This module provides methods for checking system health status.

use std::future::Future;

use reqwest::Method;

use crate::client::NvisyClient;
use crate::error::Result;
use crate::model::{CheckHealth, MonitorStatus};

/// Trait for Health API operations.
pub trait HealthService {
    /// Gets the current system health status.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nvisy_sdk::{NvisyClient, Result};
    /// use nvisy_sdk::service::HealthService;
    ///
    /// # async fn example() -> Result<()> {
    /// let client = NvisyClient::with_api_key("your-api-key")?;
    /// let status = client.health(None).await?;
    /// println!("System status: {:?} (version {})", status.status, status.version);
    /// # Ok(())
    /// # }
    /// ```
    fn health(&self, options: Option<CheckHealth>) -> impl Future<Output = Result<MonitorStatus>>;
}

impl HealthService for NvisyClient {
    async fn health(&self, options: Option<CheckHealth>) -> Result<MonitorStatus> {
        let response = match options {
            Some(opts) => self.send_json(Method::POST, "/health/", &opts).await?,
            None => self.send(Method::GET, "/health/").await?,
        };
        let response = response.error_for_status()?;
        let status: MonitorStatus = response.json().await?;
        Ok(status)
    }
}
