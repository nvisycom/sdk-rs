//! Minimal example: upload and download a file and a context.
//!
//! Run with: `cargo run --example minimal`

use nvisy_rt_sdk::model::{NewContext, NewFile, Pagination};
use nvisy_rt_sdk::service::{ContextService, FileService};
use nvisy_rt_sdk::{NvisyRt, Result};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = NvisyRt::new();

    // File round-trip
    let new_file = NewFile::from_bytes(b"Hello, world!")
        .with_filename("hello.txt")
        .with_content_type("text/plain");
    let file_id = client.upload_file(&new_file).await?;
    let file = client.download_file(file_id.id).await?;
    println!("File: {}", String::from_utf8_lossy(&file.decode_content()?));

    // Context round-trip
    let new_ctx = NewContext::new(json!({ "language": "en" }));
    let ctx_id = client.upload_context(&new_ctx).await?;
    let ctx = client.download_context(ctx_id.id).await?;
    println!("Context: {}", ctx.context);

    // List with default pagination
    let files = client.list_files(&Pagination::default()).await?;
    println!("Files: {} total", files.total);

    Ok(())
}
