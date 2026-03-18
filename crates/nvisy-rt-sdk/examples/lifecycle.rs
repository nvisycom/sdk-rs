//! Run lifecycle: create, poll, list, and cancel a redaction run.
//!
//! Run with: `cargo run --example lifecycle`

use nvisy_rt_sdk::model::NewRun;
use nvisy_rt_sdk::service::RunService;
use nvisy_rt_sdk::{NvisyRt, Result};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let client = NvisyRt::new()?;
    let actor_id = Uuid::new_v4();

    // Create a run
    let run = client.create_run(&NewRun { actor_id }).await?;
    println!("Created run: {}", run.id);

    // Poll status
    let run = client.get_run(run.id).await?;
    println!("Run: {run:?}");

    // List all runs
    let runs = client.list_runs().await?;
    println!("Total runs: {}", runs.runs.len());

    // Cancel
    client.cancel_run(run.id).await?;
    println!("Cancelled run: {}", run.id);

    Ok(())
}
