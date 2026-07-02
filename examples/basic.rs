//! Basic example: create a client and check the service health.
//!
//! Run with: `cargo run --example basic`

use nvisy_sdk::service::MonitorService;
use nvisy_sdk::{Nvisy, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("NVISY_API_KEY").expect("NVISY_API_KEY must be set");
    let client = Nvisy::with_api_key(&api_key)?;

    println!("Base URL: {}", client.base_url());
    println!("Timeout:  {:?}", client.timeout());

    // Check health
    let health = client.health(None).await?;
    println!("Status:   {:?}", health.status);
    println!("Checks:   {}", health.checks.len());
    println!("Checked:  {}", health.timestamp);

    Ok(())
}
