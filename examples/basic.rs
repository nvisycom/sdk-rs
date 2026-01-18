//! Basic example demonstrating Nvisy SDK usage.

use nvisy_sdk::{NvisyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client with an API key
    let _client = NvisyClient::with_api_key("your-api-key")?;

    // Use the client for API calls...
    // let workspaces = client.list(None).await?;

    println!("Client created successfully!");

    Ok(())
}
