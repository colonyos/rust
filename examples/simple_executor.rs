//! Simple Executor Example
//!
//! This example demonstrates how to build a basic ColonyOS executor in Rust.
//! The executor registers itself, waits for processes, and handles them.
//!
//! Run with: cargo run --example simple_executor

use colonies::core::{Executor, Log};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration - in production, load from environment
    let colonyname = "dev";
    let executor_prvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";
    let colony_prvkey = "ba949fa134981372d6da62b6a56f336ab4d843b22c02a4257dcf7d0d73097514";

    // Generate executor ID from private key
    let executor_id = colonies::crypto::gen_id(executor_prvkey);
    println!("Executor ID: {}", executor_id);

    // Create and register the executor
    let executor = Executor::new("rust-executor", &executor_id, "cli", colonyname);

    match colonies::add_executor(&executor, executor_prvkey).await {
        Ok(e) => println!("Registered executor: {}", e.executorname),
        Err(e) => {
            // Executor might already exist
            println!("Note: {}", e);
        }
    }

    // Approve the executor (requires colony owner key)
    match colonies::approve_executor(colonyname, "rust-executor", colony_prvkey).await {
        Ok(_) => println!("Executor approved"),
        Err(e) => println!("Note: {}", e),
    }

    println!("Executor running, waiting for processes...");
    println!("Submit a process with: colonies function exec --func echo --args hello --targettype cli");

    // Main processing loop
    loop {
        // Wait for a process with 10 second timeout
        match colonies::assign(colonyname, 10, executor_prvkey).await {
            Ok(process) => {
                println!("\n=== Assigned Process ===");
                println!("Process ID: {}", process.processid);
                println!("Function: {}", process.spec.funcname);
                println!("Args: {:?}", process.spec.args);

                // Log that we started processing
                let log = Log {
                    processid: process.processid.clone(),
                    colonyname: colonyname.to_string(),
                    executorname: "rust-executor".to_string(),
                    message: format!("Processing function: {}", process.spec.funcname),
                    timestamp: 0,
                };
                let _ = colonies::add_log(&log, executor_prvkey).await;

                // Handle the function
                match process.spec.funcname.as_str() {
                    "echo" => {
                        // Echo function: return the first argument
                        let output = process
                            .spec
                            .args
                            .get(0)
                            .map(|s| s.clone())
                            .unwrap_or_else(|| "no input".to_string());

                        println!("Echoing: {}", output);

                        // Set output and close successfully
                        colonies::set_output(&process.processid, vec![output], executor_prvkey)
                            .await?;
                        colonies::close(&process.processid, executor_prvkey).await?;
                        println!("Process completed successfully");
                    }
                    "add" => {
                        // Add function: add two numbers
                        if process.spec.args.len() >= 2 {
                            let a: i64 = process.spec.args[0].parse().unwrap_or(0);
                            let b: i64 = process.spec.args[1].parse().unwrap_or(0);
                            let result = a + b;

                            println!("{} + {} = {}", a, b, result);

                            colonies::set_output(
                                &process.processid,
                                vec![result.to_string()],
                                executor_prvkey,
                            )
                            .await?;
                            colonies::close(&process.processid, executor_prvkey).await?;
                            println!("Process completed successfully");
                        } else {
                            colonies::fail(&process.processid, executor_prvkey).await?;
                            println!("Process failed: add requires 2 arguments");
                        }
                    }
                    "sleep" => {
                        // Sleep function: sleep for N seconds
                        let seconds: u64 = process
                            .spec
                            .args
                            .get(0)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(1);

                        println!("Sleeping for {} seconds...", seconds);
                        tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;

                        colonies::set_output(
                            &process.processid,
                            vec![format!("Slept for {} seconds", seconds)],
                            executor_prvkey,
                        )
                        .await?;
                        colonies::close(&process.processid, executor_prvkey).await?;
                        println!("Process completed successfully");
                    }
                    _ => {
                        // Unknown function
                        println!("Unknown function: {}", process.spec.funcname);
                        colonies::fail(&process.processid, executor_prvkey).await?;
                        println!("Process marked as failed");
                    }
                }
            }
            Err(e) => {
                // Check if it's a timeout (expected) or connection error
                if e.conn_err() {
                    eprintln!("Connection error: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
                // Timeout is normal - just continue polling
            }
        }
    }
}
