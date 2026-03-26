//! Run lifecycle: create, poll, list, and cancel a redaction run.
//!
//! Run with: `cargo run --example lifecycle`

use nvisy_rt_sdk::model::{NewRun, Pagination};
use nvisy_rt_sdk::service::{RunQuery, RunService};
use nvisy_rt_sdk::{Result, Runtime};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Runtime::new();

    // Create a run
    let result = client
        .create_run(&NewRun {
            policies: json!([]),
            graph: json!({}),
            config: None,
        })
        .await?;
    println!("Created run: {}", result.run_id);

    // Get run detail
    let detail = client.get_run(result.run_id).await?;
    println!("Run detail: {detail:?}");

    // List all runs
    let runs = client
        .list_runs(&RunQuery::default(), &Pagination::default())
        .await?;
    println!("Total runs: {}", runs.total);

    // Cancel
    client.cancel_run(result.run_id).await?;
    println!("Cancelled run: {}", result.run_id);

    Ok(())
}
