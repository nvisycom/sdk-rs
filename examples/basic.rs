//! Basic example demonstrating Nvisy SDK usage.
//!
//! This example shows how to create a client and interact with the Workspaces API.
//!
//! Run with: `cargo run --example basic`

use nvisy_sdk::model::CreateWorkspace;
use nvisy_sdk::service::WorkspacesService;
use nvisy_sdk::{NvisyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client with an API key from environment
    let api_key = std::env::var("NVISY_API_KEY").expect("NVISY_API_KEY must be set");
    let client = NvisyClient::with_api_key(&api_key)?;

    // List all workspaces
    println!("Listing workspaces...");
    let workspaces = client.list_workspaces(None).await?;

    for workspace in &workspaces.items {
        println!(
            "  - {} ({})",
            workspace.display_name, workspace.workspace_id
        );
    }
    println!("Found {} workspace(s)", workspaces.items.len());

    // Create a new workspace
    println!("\nCreating a new workspace...");
    let request = CreateWorkspace::new("SDK Example Workspace")
        .with_description("Created by the Nvisy SDK example")
        .with_tags(vec!["example".into(), "sdk".into()]);

    let workspace = client.create_workspace(request).await?;
    println!(
        "Created workspace: {} ({})",
        workspace.display_name, workspace.workspace_id
    );

    // Get workspace details
    println!("\nFetching workspace details...");
    let fetched = client.get_workspace(workspace.workspace_id).await?;
    println!("  Name: {}", fetched.display_name);
    println!("  Description: {:?}", fetched.description);
    println!("  Tags: {:?}", fetched.tags);
    println!("  Role: {:?}", fetched.member_role);

    // Delete the workspace
    println!("\nCleaning up - deleting workspace...");
    client.delete_workspace(workspace.workspace_id).await?;
    println!("Workspace deleted successfully");

    Ok(())
}
