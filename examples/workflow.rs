//! Workflow Example
//!
//! This example demonstrates how to submit a workflow (DAG of processes)
//! where processes can depend on each other.
//!
//! Run with: cargo run --example workflow

use colonies::core::{FunctionSpec, WorkflowSpec, SUCCESS, FAILED};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let colonyname = "dev";
    let prvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";

    println!("=== Submitting a Workflow ===\n");

    // Create a workflow with 3 steps:
    // step1 and step2 run in parallel
    // step3 depends on both step1 and step2

    // Step 1: First parallel task
    let mut step1 = FunctionSpec::new("echo", "cli", colonyname);
    step1.nodename = "step1".to_string();
    step1.args = vec!["Step 1 complete".to_string()];
    step1.maxexectime = 60;

    // Step 2: Second parallel task
    let mut step2 = FunctionSpec::new("echo", "cli", colonyname);
    step2.nodename = "step2".to_string();
    step2.args = vec!["Step 2 complete".to_string()];
    step2.maxexectime = 60;

    // Step 3: Depends on step1 and step2
    let mut step3 = FunctionSpec::new("echo", "cli", colonyname);
    step3.nodename = "step3".to_string();
    step3.args = vec!["Step 3 complete - all done!".to_string()];
    step3.maxexectime = 60;
    step3.conditions.dependencies = vec!["step1".to_string(), "step2".to_string()];

    // Create the workflow spec
    let workflow = WorkflowSpec {
        colonyname: colonyname.to_string(),
        functionspecs: vec![step1, step2, step3],
    };

    // Submit the workflow
    let processgraph = colonies::submit_workflow(&workflow, prvkey).await?;
    println!("Submitted workflow: {}", processgraph.processgraphid);
    println!("Root processes: {:?}", processgraph.rootprocessids);
    println!("All processes: {:?}", processgraph.processids);
    println!();

    // Wait for the workflow to complete
    println!("Waiting for workflow completion...\n");

    loop {
        let pg = colonies::get_processgraph(&processgraph.processgraphid, prvkey).await?;

        match pg.state {
            s if s == SUCCESS => {
                println!("=== Workflow Completed Successfully ===");

                // Get details of each process
                for pid in &pg.processids {
                    let p = colonies::get_process(pid, prvkey).await?;
                    println!(
                        "  {} ({}): {:?}",
                        p.spec.nodename,
                        if p.state == SUCCESS { "SUCCESS" } else { "FAILED" },
                        p.output
                    );
                }
                break;
            }
            s if s == FAILED => {
                println!("=== Workflow Failed ===");

                // Show which processes failed
                for pid in &pg.processids {
                    let p = colonies::get_process(pid, prvkey).await?;
                    if p.state == FAILED {
                        println!("  {} failed: {:?}", p.spec.nodename, p.errors);
                    }
                }
                break;
            }
            _ => {
                // Still in progress
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
    }

    // Clean up
    colonies::remove_processgraph(&processgraph.processgraphid, prvkey).await?;
    println!("\nWorkflow removed");

    Ok(())
}
