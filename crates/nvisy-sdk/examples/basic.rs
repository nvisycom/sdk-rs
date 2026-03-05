//! Basic example demonstrating Nvisy SDK usage.
//!
//! This example shows how to create a client and check the service health.
//!
//! Run with: `cargo run --example basic`

use nvisy_sdk::service::MonitorService;
use nvisy_sdk::{Nvisy, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("NVISY_API_KEY").expect("NVISY_API_KEY must be set");
    let client = Nvisy::with_api_key(&api_key)?;

    println!("Nvisy client created successfully");
    println!("  Base URL: {}", client.base_url());
    println!("  Timeout: {:?}", client.timeout());

    // Check health status
    let status = client.health_status(None).await?;
    println!("  Health: {:?}", status.status);
    println!("  Version: {}", status.version);
    println!("  Checked at: {}", status.checked_at);

    Ok(())
}
