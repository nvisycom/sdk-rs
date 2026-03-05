//! File upload and management example.
//!
//! This example demonstrates uploading, listing, and downloading files.
//!
//! Run with: `cargo run --example files`

use std::fs;

use nvisy_sdk::model::{ArchiveFormat, CreateWorkspace};
use nvisy_sdk::service::{FilesService, ListFilesOptions, WorkspacesService};
use nvisy_sdk::{NvisyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client
    let api_key = std::env::var("NVISY_API_KEY").expect("NVISY_API_KEY must be set");
    let client = NvisyClient::with_api_key(&api_key)?;

    // Create a workspace for our files
    println!("Creating workspace for file uploads...");
    let workspace = client
        .create_workspace(CreateWorkspace::new("File Upload Example"))
        .await?;
    let workspace_id = workspace.workspace_id;
    println!("Created workspace: {}", workspace_id);

    // Upload a file
    println!("\nUploading file...");
    let content = b"Hello from the Nvisy SDK!\n\nThis is a test document.".to_vec();
    let file = client
        .upload_file(workspace_id, "hello.txt", content)
        .await?;
    println!("Uploaded: {} ({} bytes)", file.display_name, file.file_size);
    println!("  Status: {:?}", file.status);
    println!("  File ID: {}", file.file_id);

    // Upload another file
    println!("\nUploading another file...");
    let content2 = b"# README\n\nThis is another test file.".to_vec();
    let file2 = client
        .upload_file(workspace_id, "readme.md", content2)
        .await?;
    println!(
        "Uploaded: {} ({} bytes)",
        file2.display_name, file2.file_size
    );

    // List files in the workspace
    println!("\nListing files in workspace...");
    let options = ListFilesOptions::new().limit(10);
    let files = client.list_files(workspace_id, Some(options)).await?;
    for f in &files.items {
        println!("  - {} ({:?})", f.display_name, f.status);
    }
    println!("Total: {} file(s)", files.items.len());

    // Download a single file
    println!("\nDownloading single file...");
    let downloaded = client.download_file(file.file_id).await?;
    println!(
        "Downloaded {} bytes: {:?}",
        downloaded.len(),
        String::from_utf8_lossy(&downloaded)
    );

    // Download all files as ZIP archive
    println!("\nDownloading all files as ZIP...");
    let archive = client
        .download_files_batch(workspace_id, vec![], ArchiveFormat::Zip)
        .await?;
    println!("Downloaded archive: {} bytes", archive.len());

    // Optionally save the archive
    if std::env::var("SAVE_ARCHIVE").is_ok() {
        fs::write("files.zip", &archive)?;
        println!("Saved to files.zip");
    }

    // Delete files
    println!("\nDeleting files...");
    client.delete_file(file.file_id).await?;
    client.delete_file(file2.file_id).await?;
    println!("Files deleted");

    // Cleanup workspace
    println!("\nCleaning up workspace...");
    client.delete_workspace(workspace_id).await?;
    println!("Done!");

    Ok(())
}
