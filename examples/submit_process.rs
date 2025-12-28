//! Submit Process Example
//!
//! This example demonstrates how to submit a process and wait for its completion.
//!
//! Run with: cargo run --example submit_process

use colonyos::core::{FunctionSpec, SUCCESS, FAILED, WAITING, RUNNING};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let colonyname = "dev";
    let prvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";

    println!("=== Submitting a process ===\n");

    // Create a function spec
    let mut spec = FunctionSpec::new("echo", "cli", colonyname);
    spec.args = vec!["Hello from Rust SDK!".to_string()];
    spec.maxexectime = 60;  // Max 60 seconds to execute
    spec.maxwaittime = 300; // Max 5 minutes waiting in queue
    spec.maxretries = 3;    // Retry up to 3 times on failure

    // Add environment variables
    spec.env.insert("MY_VAR".to_string(), "my_value".to_string());

    // Submit the process
    let process = colonyos::submit(&spec, prvkey).await?;
    println!("Submitted process: {}", process.processid);
    println!("Function: {}", spec.funcname);
    println!("Args: {:?}", spec.args);
    println!("State: WAITING\n");

    // Wait for completion
    println!("Waiting for completion...");
    let mut last_state = WAITING;

    loop {
        let p = colonyos::get_process(&process.processid, prvkey).await?;

        // Print state changes
        if p.state != last_state {
            match p.state {
                WAITING => println!("State: WAITING"),
                RUNNING => println!("State: RUNNING (assigned to executor)"),
                SUCCESS => println!("State: SUCCESS"),
                FAILED => println!("State: FAILED"),
                _ => println!("State: {}", p.state),
            }
            last_state = p.state;
        }

        match p.state {
            SUCCESS => {
                println!("\n=== Process Completed Successfully ===");
                println!("Output: {:?}", p.output);
                if !p.attributes.is_empty() {
                    println!("Attributes:");
                    for attr in &p.attributes {
                        println!("  {}: {}", attr.key, attr.value);
                    }
                }
                break;
            }
            FAILED => {
                println!("\n=== Process Failed ===");
                println!("Errors: {:?}", p.errors);
                break;
            }
            _ => {
                // Still waiting or running
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
    }

    // Clean up - remove the process
    colonyos::remove_process(&process.processid, prvkey).await?;
    println!("\nProcess removed");

    Ok(())
}
