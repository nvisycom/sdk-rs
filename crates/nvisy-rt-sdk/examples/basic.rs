//! Basic example demonstrating Nvisy Runtime SDK usage.
//!
//! This example shows how to create a runtime client.
//!
//! Run with: `cargo run --example basic`

use nvisy_rt_sdk::{NvisyRt, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = NvisyRt::new()?;

    println!("Nvisy Runtime client created successfully");
    println!("  Base URL: {}", client.base_url());
    println!("  Timeout: {:?}", client.timeout());

    Ok(())
}
