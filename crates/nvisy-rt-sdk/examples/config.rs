//! Custom client configuration and health check.
//!
//! Run with: `cargo run --example config`

use std::time::Duration;

use nvisy_rt_sdk::service::CheckService;
use nvisy_rt_sdk::{NvisyRt, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = NvisyRt::builder()
        .with_base_url("http://localhost:9090")
        .with_timeout(Duration::from_secs(10))
        .with_max_retries(5u32)
        .with_user_agent("my-app/1.0")
        .build()?;

    println!("Base URL: {}", client.base_url());
    println!("Timeout:  {:?}", client.timeout());

    // Health check
    let health = client.health().await?;
    println!("Healthy:  {}", health.healthy);

    // Analytics
    let analytics = client.analytics().await?;
    println!("Analytics: {analytics:?}");

    Ok(())
}
