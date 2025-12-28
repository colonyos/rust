# ColonyOS Rust SDK

[![Rust](https://github.com/colonyos/rust/actions/workflows/rust.yml/badge.svg)](https://github.com/colonyos/rust/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/colonyos/rust/branch/main/graph/badge.svg)](https://codecov.io/gh/colonyos/rust)
[![Crates.io](https://img.shields.io/crates/v/colonies.svg)](https://crates.io/crates/colonies)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust SDK for [ColonyOS](https://github.com/colonyos/colonies) - build distributed applications with executors that can run anywhere.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
colonies = "0.1"
```

## Quick Start

```rust
use colonies::core::{Attribute, FunctionSpec};

#[tokio::main]
async fn main() {
    let colonyname = "dev";
    let executorprvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";

    loop {
        // Wait for a process to be assigned
        let process = colonies::assign(colonyname, false, 10, executorprvkey)
            .await
            .unwrap();

        match process.spec.funcname.as_str() {
            "say" => {
                // Add output attribute
                let attr = Attribute::new(
                    colonyname,
                    &process.processid,
                    "output",
                    &process.spec.args[0],
                );
                colonies::add_attr(&attr, executorprvkey).await.unwrap();

                // Close the process successfully
                colonies::close(&process.processid, executorprvkey).await.unwrap();
            }
            _ => {
                // Fail unknown functions
                colonies::fail(&process.processid, vec![], executorprvkey)
                    .await
                    .unwrap();
            }
        }
    }
}
```

## Features

- Pure Rust implementation (no C dependencies)
- Async/await support with Tokio
- Full ColonyOS API support:
  - Colony management
  - Executor registration and management
  - Process submission, assignment, and completion
  - Workflow (process graph) support
  - Logging
  - Channel communication
  - Blueprint management and reconciliation
  - Function registration

## Running Tests

### Start a Colonies server

```bash
source devenv
colonies dev
```

### Run tests

```bash
cargo test
```

## Example: Submit a Process

```rust
use colonies::core::FunctionSpec;

#[tokio::main]
async fn main() {
    let colonyname = "dev";
    let prvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";

    // Create a function spec
    let mut spec = FunctionSpec::new("say", "cli", colonyname);
    spec.args = vec!["hello".to_string()];

    // Submit the process
    let process = colonies::submit(&spec, prvkey).await.unwrap();
    println!("Submitted process: {}", process.processid);
}
```

## License

MIT
