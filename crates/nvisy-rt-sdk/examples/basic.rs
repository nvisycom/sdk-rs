//! Basic example demonstrating Nvisy Runtime SDK usage.
//!
//! This example shows how to create a runtime client and use the
//! file and context services.
//!
//! Run with: `cargo run --example basic`

use nvisy_rt_sdk::model::{NewContext, NewFile};
use nvisy_rt_sdk::service::{ContextService, FileService};
use nvisy_rt_sdk::{NvisyRt, Result};
use serde_json::json;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let client = NvisyRt::new()?;
    let actor_id = Uuid::new_v4();

    println!("Nvisy Runtime client created successfully");
    println!("  Base URL: {}", client.base_url());
    println!("  Timeout: {:?}", client.timeout());

    // Upload a file
    let new_file = NewFile::from_bytes(actor_id, b"Hello, world!")
        .with_filename("hello.txt")
        .with_content_type("text/plain");
    let file_id = client.upload_file(&new_file).await?;
    println!("  Uploaded file: {}", file_id.id);

    // Download the file
    let file = client.download_file(file_id.id).await?;
    let bytes = file.decode_content()?;
    println!("  File content: {}", String::from_utf8_lossy(&bytes));

    // List all files
    let files = client.list_files().await?;
    println!("  Total files: {}", files.files.len());

    // Create a context
    let new_context = NewContext::new(actor_id, json!({ "language": "en" }));
    let context_id = client.upload_context(&new_context).await?;
    println!("  Uploaded context: {}", context_id.id);

    // Download the context
    let context = client.download_context(context_id.id).await?;
    println!("  Context value: {}", context.context);

    // List all contexts
    let contexts = client.list_contexts().await?;
    println!("  Total contexts: {}", contexts.contexts.len());

    Ok(())
}
