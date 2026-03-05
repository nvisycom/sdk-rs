//! Basic example demonstrating Nvisy Runtime SDK usage.
//!
//! This example shows how to create a runtime client.
//!
//! Run with: `cargo run --example basic`

use nvisy_rt_sdk::{NvisyRt, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client with an API key from environment
    let api_key = std::env::var("NVISY_API_KEY").expect("NVISY_API_KEY must be set");
    let client = NvisyRt::with_api_key(&api_key)?;

    println!("Nvisy Runtime client created successfully");
    println!("  Base URL: {}", client.config().base_url());
    println!("  Timeout: {:?}", client.config().timeout());

    Ok(())
}
