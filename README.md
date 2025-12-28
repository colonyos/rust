# ColonyOS Rust SDK

[![Rust](https://github.com/colonyos/rust/actions/workflows/rust.yml/badge.svg)](https://github.com/colonyos/rust/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/colonyos/rust/branch/main/graph/badge.svg)](https://codecov.io/gh/colonyos/rust)
[![Crates.io](https://img.shields.io/crates/v/colonies.svg)](https://crates.io/crates/colonies)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust SDK for [ColonyOS](https://github.com/colonyos/colonies) - build distributed applications with executors that can run anywhere.

## Features

- Pure Rust implementation (no C dependencies)
- Async/await support with Tokio
- secp256k1 ECDSA cryptography with SHA3-256 hashing
- Full ColonyOS API support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
colonies = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Building an Executor

An executor is a worker that pulls and executes tasks from a ColonyOS server:

```rust
use colonyos::core::{Attribute, Executor, FunctionSpec, WAITING};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let colonyname = "dev";
    let prvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";

    // Create and register executor
    let executor_id = colonyos::crypto::gen_id(prvkey);
    let executor = Executor::new("rust-executor", &executor_id, "cli", colonyname);
    colonyos::add_executor(&executor, prvkey).await?;

    println!("Executor registered, waiting for processes...");

    loop {
        // Wait for a process (10 second timeout)
        match colonyos::assign(colonyname, 10, prvkey).await {
            Ok(process) => {
                println!("Assigned process: {}", process.processid);

                match process.spec.funcname.as_str() {
                    "echo" => {
                        // Echo back the first argument
                        let output = process.spec.args.get(0)
                            .map(|s| s.clone())
                            .unwrap_or_default();
                        colonyos::set_output(&process.processid, vec![output], prvkey).await?;
                        colonyos::close(&process.processid, prvkey).await?;
                        println!("Process completed successfully");
                    }
                    _ => {
                        colonyos::fail(&process.processid, prvkey).await?;
                        println!("Unknown function: {}", process.spec.funcname);
                    }
                }
            }
            Err(e) => {
                // Timeout or connection error - continue polling
                if !e.conn_err() {
                    continue;
                }
                eprintln!("Connection error: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}
```

### Submitting a Process

```rust
use colonyos::core::FunctionSpec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";

    // Create and submit a function spec
    let mut spec = FunctionSpec::new("echo", "cli", "dev");
    spec.args = vec!["Hello from Rust!".to_string()];
    spec.maxexectime = 60;
    spec.maxretries = 3;

    let process = colonyos::submit(&spec, prvkey).await?;
    println!("Submitted process: {}", process.processid);

    // Wait for completion
    loop {
        let p = colonyos::get_process(&process.processid, prvkey).await?;
        match p.state {
            colonyos::core::SUCCESS => {
                println!("Output: {:?}", p.output);
                break;
            }
            colonyos::core::FAILED => {
                println!("Process failed");
                break;
            }
            _ => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }

    Ok(())
}
```

## API Overview

See [API.md](API.md) for the complete API reference.

### Colony Management
- `add_colony`, `remove_colony`, `get_colony`, `get_colonies`

### Executor Management
- `add_executor`, `approve_executor`, `reject_executor`, `remove_executor`
- `get_executor`, `get_executors`

### Process Lifecycle
- `submit` - Submit a new process
- `assign` - Wait for and assign a process to execute
- `close` - Mark process as successful
- `fail` - Mark process as failed
- `get_process`, `get_processes`, `remove_process`, `remove_all_processes`
- `set_output` - Set process output

### Workflows
- `submit_workflow`, `get_processgraph`, `get_processgraphs`
- `remove_processgraph`, `remove_all_processgraphs`

### Logging
- `add_log`, `get_logs`

### Channels
- `channel_append`, `channel_read`

### Statistics
- `get_statistics`

### Function Registry
- `add_function`, `get_functions`, `get_functions_by_executor`, `remove_function`

### Blueprints
- `add_blueprint_definition`, `get_blueprint_definition`, `get_blueprint_definitions`, `remove_blueprint_definition`
- `add_blueprint`, `get_blueprint`, `get_blueprints`, `update_blueprint`, `remove_blueprint`
- `update_blueprint_status`, `reconcile_blueprint`

## Cryptography

The SDK uses pure Rust cryptography:

```rust
use colonyos::crypto;

// Generate a new private key
let prvkey = crypto::gen_prvkey();

// Derive the public ID from a private key
let id = crypto::gen_id(&prvkey);

// Sign a message
let signature = crypto::gen_signature("message", &prvkey);

// Verify signature and recover ID
let recovered_id = crypto::recid("message", &signature);
assert_eq!(id, recovered_id);

// Hash a message with SHA3-256
let hash = crypto::gen_hash("message");
```

## Running Tests

### Start a Colonies server

```bash
cd /path/to/colonies
source docker-compose.env
docker-compose up -d
```

### Run tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run integration tests (requires running server)
cargo test --test integration_test
```

## Examples

See the [examples](examples/) directory:

- `examples/simple_executor.rs` - Basic executor implementation
- `examples/submit_process.rs` - Submit and monitor a process
- `examples/workflow.rs` - Submit a workflow with dependencies

## Tutorial

See [tutorials/getting-started.md](tutorials/getting-started.md) for a step-by-step guide.

## License

MIT
