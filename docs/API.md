# ColonyOS Rust SDK API Reference

This document provides a complete reference for the ColonyOS Rust SDK.

## Table of Contents

- [Core Types](#core-types)
- [Constants](#constants)
- [Cryptography](#cryptography)
- [Colony Management](#colony-management)
- [Executor Management](#executor-management)
- [Process Management](#process-management)
- [Workflow Management](#workflow-management)
- [Logging](#logging)
- [Channels](#channels)
- [Statistics](#statistics)
- [Function Registry](#function-registry)
- [Blueprint Management](#blueprint-management)
- [Error Handling](#error-handling)

---

## Core Types

### Colony

```rust
pub struct Colony {
    pub colonyid: String,
    pub name: String,
}

impl Colony {
    pub fn new(colonyid: &str, name: &str) -> Colony;
}
```

### Executor

```rust
pub struct Executor {
    pub executorid: String,
    pub executortype: String,
    pub executorname: String,
    pub colonyname: String,
    pub state: i32,
    pub requirefuncreg: bool,
    pub commissiontime: String,
    pub lastheardfromtime: String,
    pub locationname: String,
    pub capabilities: Capabilities,
    pub allocations: Allocations,
}

impl Executor {
    pub fn new(name: &str, executorid: &str, executortype: &str, colonyname: &str) -> Executor;
}
```

### FunctionSpec

Defines the specification for a process to be executed.

```rust
pub struct FunctionSpec {
    pub nodename: String,           // Name for workflow dependencies
    pub funcname: String,           // Function name
    pub args: Vec<String>,          // Positional arguments
    pub kwargs: HashMap<String, Value>, // Keyword arguments
    pub priority: i32,              // Higher priority = executed first
    pub maxwaittime: i32,           // Max seconds waiting in queue
    pub maxexectime: i32,           // Max seconds for execution
    pub maxretries: i32,            // Max retry attempts
    pub conditions: Conditions,     // Execution conditions
    pub label: String,              // Optional label
    pub fs: Filesystem,             // Filesystem configuration
    pub env: HashMap<String, String>, // Environment variables
    pub channels: Vec<String>,      // Channel names
}

impl FunctionSpec {
    pub fn new(funcname: &str, executortype: &str, colonyname: &str) -> FunctionSpec;
}
```

### Conditions

Specifies conditions for process assignment.

```rust
pub struct Conditions {
    pub colonyname: String,
    pub executornames: Vec<String>,  // Specific executors (optional)
    pub executortype: String,        // Required executor type
    pub dependencies: Vec<String>,   // Workflow dependencies (node names)
    pub nodes: i32,                  // Number of nodes required
    pub cpu: String,                 // CPU requirements
    pub processes: i32,
    pub processes_per_node: i32,
    pub mem: String,                 // Memory requirements
    pub storage: String,             // Storage requirements
    pub gpu: GPU,                    // GPU requirements
    pub walltime: i64,               // Wall time limit
}
```

### Process

Represents a submitted process.

```rust
pub struct Process {
    pub processid: String,
    pub initiatorid: String,
    pub initiatorname: String,
    pub assignedexecutorid: String,
    pub isassigned: bool,
    pub state: i32,                 // WAITING, RUNNING, SUCCESS, FAILED
    pub prioritytime: i64,
    pub submissiontime: String,
    pub starttime: String,
    pub endtime: String,
    pub retries: i32,
    pub attributes: Vec<Attribute>,
    pub spec: FunctionSpec,
    pub waitforparents: bool,
    pub parents: Vec<String>,
    pub children: Vec<String>,
    pub processgraphid: String,
    pub input: Vec<String>,
    pub output: Vec<String>,
    pub errors: Vec<String>,
}
```

### ProcessGraph

Represents a workflow (DAG of processes).

```rust
pub struct ProcessGraph {
    pub processgraphid: String,
    pub colonyname: String,
    pub state: i32,
    pub rootprocessids: Vec<String>,
    pub processids: Vec<String>,
}
```

### WorkflowSpec

Specification for submitting a workflow.

```rust
pub struct WorkflowSpec {
    pub colonyname: String,
    pub functionspecs: Vec<FunctionSpec>,
}
```

### Attribute

Key-value attribute attached to a process.

```rust
pub struct Attribute {
    pub attributeid: String,
    pub targetid: String,
    pub targetcolonyname: String,
    pub targetprocessgraphid: String,
    pub attributetype: i32,  // IN, OUT, ERR, ENV
    pub key: String,
    pub value: String,
}

impl Attribute {
    pub fn new(colonyname: &str, processid: &str, key: &str, value: &str) -> Attribute;
}
```

### Log

Log message for a process.

```rust
pub struct Log {
    pub processid: String,
    pub colonyname: String,
    pub executorname: String,
    pub message: String,
    pub timestamp: i64,
}
```

### ChannelEntry

Entry in a process channel.

```rust
pub struct ChannelEntry {
    pub sequence: i64,
    pub data: String,
    pub msgtype: String,
    pub inreplyto: i64,
}
```

### Statistics

Colony statistics.

```rust
pub struct Statistics {
    pub colonies: i64,
    pub executors: i64,
    pub waitingprocesses: i64,
    pub runningprocesses: i64,
    pub successfulprocesses: i64,
    pub failedprocesses: i64,
    pub waitingworkflows: i64,
    pub runningworkflows: i64,
    pub successfulworkflows: i64,
    pub failedworkflows: i64,
}
```

### Function

Registered function metadata.

```rust
pub struct Function {
    pub functionid: String,
    pub executorname: String,
    pub executortype: String,
    pub colonyname: String,
    pub funcname: String,
    pub counter: i64,
    pub minwaittime: f64,
    pub maxwaittime: f64,
    pub minexectime: f64,
    pub maxexectime: f64,
    pub avgwaittime: f64,
    pub avgexectime: f64,
}
```

### Blueprint

Blueprint for reconciliation.

```rust
pub struct Blueprint {
    pub blueprintid: String,
    pub kind: String,
    pub metadata: BlueprintMetadata,
    pub handler: BlueprintHandler,
    pub spec: HashMap<String, Value>,
    pub status: HashMap<String, Value>,
    pub generation: i64,
    pub reconciledgeneration: i64,
    pub lastreconciled: String,
}
```

---

## Constants

### Process States

```rust
pub const WAITING: i32 = 0;  // Process waiting in queue
pub const RUNNING: i32 = 1;  // Process being executed
pub const SUCCESS: i32 = 2;  // Process completed successfully
pub const FAILED: i32 = 3;   // Process failed
```

### Executor States

```rust
pub const PENDING: i32 = 0;   // Executor pending approval
pub const APPROVED: i32 = 1;  // Executor approved
pub const REJECTED: i32 = 2;  // Executor rejected
```

### Attribute Types

```rust
pub const IN: i32 = 0;   // Input attribute
pub const OUT: i32 = 1;  // Output attribute
pub const ERR: i32 = 2;  // Error attribute
pub const ENV: i32 = 4;  // Environment attribute
```

---

## Cryptography

All functions are in the `crypto` module.

### gen_prvkey

Generate a new random private key.

```rust
pub fn gen_prvkey() -> String
```

Returns a hex-encoded 32-byte private key (64 characters).

### gen_id

Derive the public ID from a private key.

```rust
pub fn gen_id(private_key: &str) -> String
```

Returns a hex-encoded SHA3-256 hash of the public key (64 characters).

### gen_signature

Sign a message with a private key.

```rust
pub fn gen_signature(message: &str, private_key: &str) -> String
```

Returns a hex-encoded signature (130 characters: r + s + v).

### gen_hash

Hash a message with SHA3-256.

```rust
pub fn gen_hash(message: &str) -> String
```

Returns a hex-encoded hash (64 characters).

### recid

Recover the public ID from a message and signature.

```rust
pub fn recid(message: &str, signature: &str) -> String
```

Returns the hex-encoded ID that signed the message.

---

## Colony Management

### add_colony

Create a new colony.

```rust
pub async fn add_colony(
    colony: &Colony,
    prvkey: &str,
) -> Result<Colony, RPCError>
```

### remove_colony

Delete a colony.

```rust
pub async fn remove_colony(
    colony_name: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### get_colony

Get a colony by name.

```rust
pub async fn get_colony(
    colonyname: &str,
    prvkey: &str,
) -> Result<Colony, RPCError>
```

### get_colonies

Get all colonies.

```rust
pub async fn get_colonies(
    prvkey: &str,
) -> Result<Vec<Colony>, RPCError>
```

---

## Executor Management

### add_executor

Register a new executor.

```rust
pub async fn add_executor(
    executor: &Executor,
    prvkey: &str,
) -> Result<Executor, RPCError>
```

### approve_executor

Approve a pending executor (requires colony owner key).

```rust
pub async fn approve_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### reject_executor

Reject a pending executor.

```rust
pub async fn reject_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### remove_executor

Remove an executor.

```rust
pub async fn remove_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### get_executor

Get an executor by name.

```rust
pub async fn get_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<Executor, RPCError>
```

### get_executors

Get all executors in a colony.

```rust
pub async fn get_executors(
    colonyname: &str,
    prvkey: &str,
) -> Result<Vec<Executor>, RPCError>
```

---

## Process Management

### submit

Submit a new process for execution.

```rust
pub async fn submit(
    spec: &FunctionSpec,
    prvkey: &str,
) -> Result<Process, RPCError>
```

### assign

Wait for and assign a process to execute.

```rust
pub async fn assign(
    colonyname: &str,
    timeout: i32,  // Timeout in seconds
    prvkey: &str,
) -> Result<Process, RPCError>
```

**Note:** Returns an error with `conn_err() == false` on timeout.

### close

Mark a process as successfully completed.

```rust
pub async fn close(
    processid: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### fail

Mark a process as failed.

```rust
pub async fn fail(
    processid: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### get_process

Get a process by ID.

```rust
pub async fn get_process(
    processid: &str,
    prvkey: &str,
) -> Result<Process, RPCError>
```

### get_processes

Get processes by state.

```rust
pub async fn get_processes(
    colonyname: &str,
    count: i32,
    state: i32,  // WAITING, RUNNING, SUCCESS, or FAILED
    prvkey: &str,
) -> Result<Vec<Process>, RPCError>
```

### remove_process

Remove a process.

```rust
pub async fn remove_process(
    processid: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### remove_all_processes

Remove all processes with a given state.

```rust
pub async fn remove_all_processes(
    colonyname: &str,
    state: i32,
    prvkey: &str,
) -> Result<(), RPCError>
```

### set_output

Set the output of a process.

```rust
pub async fn set_output(
    processid: &str,
    output: Vec<String>,
    prvkey: &str,
) -> Result<(), RPCError>
```

### add_attr

Add an attribute to a process.

```rust
pub async fn add_attr(
    attr: &Attribute,
    prvkey: &str,
) -> Result<Attribute, RPCError>
```

---

## Workflow Management

### submit_workflow

Submit a workflow (DAG of processes).

```rust
pub async fn submit_workflow(
    workflowspec: &WorkflowSpec,
    prvkey: &str,
) -> Result<ProcessGraph, RPCError>
```

### get_processgraph

Get a process graph by ID.

```rust
pub async fn get_processgraph(
    processgraphid: &str,
    prvkey: &str,
) -> Result<ProcessGraph, RPCError>
```

### get_processgraphs

Get process graphs by state.

```rust
pub async fn get_processgraphs(
    colonyname: &str,
    count: i32,
    state: i32,
    prvkey: &str,
) -> Result<Vec<ProcessGraph>, RPCError>
```

### remove_processgraph

Remove a process graph.

```rust
pub async fn remove_processgraph(
    processgraphid: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### remove_all_processgraphs

Remove all process graphs with a given state.

```rust
pub async fn remove_all_processgraphs(
    colonyname: &str,
    state: i32,
    prvkey: &str,
) -> Result<(), RPCError>
```

---

## Logging

### add_log

Add a log message for a process.

```rust
pub async fn add_log(
    log: &Log,
    prvkey: &str,
) -> Result<(), RPCError>
```

### get_logs

Get logs for a process.

```rust
pub async fn get_logs(
    colonyname: &str,
    processid: &str,
    executorname: &str,
    count: i32,
    since: i64,  // Timestamp
    prvkey: &str,
) -> Result<Vec<Log>, RPCError>
```

---

## Channels

Channels provide real-time communication with processes. A process must have channels
defined in its FunctionSpec before they can be used.

### ChannelEntry

```rust
pub struct ChannelEntry {
    pub sequence: i64,      // Message sequence number
    pub payload: String,    // Base64 encoded payload
    pub msgtype: String,    // Message type ("data", "end", or "error")
    pub inreplyto: i64,     // Sequence number this replies to
    pub timestamp: String,  // ISO 8601 timestamp
    pub senderid: String,   // Sender's executor ID
}

impl ChannelEntry {
    /// Returns the payload decoded from base64 as a UTF-8 string
    pub fn payload_as_string(&self) -> String;

    /// Returns the raw payload bytes decoded from base64
    pub fn payload_bytes(&self) -> Vec<u8>;
}
```

### channel_append

Append data to a process channel.

```rust
pub async fn channel_append(
    processid: &str,
    channelname: &str,
    sequence: i64,       // Client-assigned sequence number
    data: &str,          // Message content
    data_type: &str,     // Empty string, "end", or "error"
    inreplyto: i64,      // Sequence number this replies to (0 if not a reply)
    prvkey: &str,
) -> Result<ChannelEntry, RPCError>
```

### channel_read

Read from a process channel.

```rust
pub async fn channel_read(
    processid: &str,
    channelname: &str,
    afterseq: i64,       // Read messages after this sequence (0 for all)
    limit: i32,          // Max messages to return (0 for no limit)
    prvkey: &str,
) -> Result<Vec<ChannelEntry>, RPCError>
```

**Example:**

```rust
// Create a spec with a channel
let mut spec = FunctionSpec::new("my_func", "cli", "dev");
spec.channels = vec!["output".to_string()];

// Submit and assign the process
let process = colonyos::submit(&spec, &prvkey).await?;
let assigned = colonyos::assign(&colonyname, 10, &prvkey).await?;

// Append messages to the channel
colonyos::channel_append(
    &assigned.processid,
    "output",
    1,           // sequence
    "Hello!",
    "",          // payloadtype
    0,           // inreplyto
    &prvkey,
).await?;

// Read messages from the channel
let messages = colonyos::channel_read(
    &assigned.processid,
    "output",
    0,   // afterseq (0 = all)
    10,  // limit
    &prvkey,
).await?;

for msg in messages {
    println!("Message {}: {}", msg.sequence, msg.payload_as_string());
}
```

---

## Statistics

### get_statistics

Get statistics for a colony.

```rust
pub async fn get_statistics(
    colonyname: &str,
    prvkey: &str,
) -> Result<Statistics, RPCError>
```

---

## Function Registry

### add_function

Register a function.

```rust
pub async fn add_function(
    function: &Function,
    prvkey: &str,
) -> Result<Function, RPCError>
```

### get_functions

Get all functions in a colony.

```rust
pub async fn get_functions(
    colonyname: &str,
    prvkey: &str,
) -> Result<Vec<Function>, RPCError>
```

### get_functions_by_executor

Get functions registered by a specific executor.

```rust
pub async fn get_functions_by_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<Vec<Function>, RPCError>
```

### remove_function

Remove a function.

```rust
pub async fn remove_function(
    functionid: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

---

## Blueprint Management

### add_blueprint_definition

Add a blueprint definition.

```rust
pub async fn add_blueprint_definition(
    definition: &BlueprintDefinition,
    prvkey: &str,
) -> Result<BlueprintDefinition, RPCError>
```

### get_blueprint_definition

Get a blueprint definition.

```rust
pub async fn get_blueprint_definition(
    colonyname: &str,
    name: &str,
    prvkey: &str,
) -> Result<BlueprintDefinition, RPCError>
```

### get_blueprint_definitions

Get all blueprint definitions.

```rust
pub async fn get_blueprint_definitions(
    colonyname: &str,
    prvkey: &str,
) -> Result<Vec<BlueprintDefinition>, RPCError>
```

### remove_blueprint_definition

Remove a blueprint definition.

```rust
pub async fn remove_blueprint_definition(
    colonyname: &str,
    name: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### add_blueprint

Add a blueprint.

```rust
pub async fn add_blueprint(
    blueprint: &Blueprint,
    prvkey: &str,
) -> Result<Blueprint, RPCError>
```

### get_blueprint

Get a blueprint.

```rust
pub async fn get_blueprint(
    colonyname: &str,
    name: &str,
    prvkey: &str,
) -> Result<Blueprint, RPCError>
```

### get_blueprints

Get blueprints by kind and location.

```rust
pub async fn get_blueprints(
    colonyname: &str,
    kind: &str,
    location: &str,
    prvkey: &str,
) -> Result<Vec<Blueprint>, RPCError>
```

### update_blueprint

Update a blueprint.

```rust
pub async fn update_blueprint(
    blueprint: &Blueprint,
    force_generation: bool,
    prvkey: &str,
) -> Result<Blueprint, RPCError>
```

### remove_blueprint

Remove a blueprint.

```rust
pub async fn remove_blueprint(
    colonyname: &str,
    name: &str,
    prvkey: &str,
) -> Result<(), RPCError>
```

### update_blueprint_status

Update the status of a blueprint.

```rust
pub async fn update_blueprint_status(
    colonyname: &str,
    name: &str,
    status: HashMap<String, Value>,
    prvkey: &str,
) -> Result<(), RPCError>
```

### reconcile_blueprint

Trigger reconciliation for a blueprint.

```rust
pub async fn reconcile_blueprint(
    colonyname: &str,
    name: &str,
    force: bool,
    prvkey: &str,
) -> Result<Process, RPCError>
```

---

## Error Handling

### RPCError

```rust
pub struct RPCError {
    // Private fields
}

impl RPCError {
    /// Returns true if this was a connection error
    pub fn conn_err(&self) -> bool;
}

impl std::fmt::Display for RPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

impl std::error::Error for RPCError {}
```

### Example Error Handling

```rust
match colonyos::assign(colonyname, 10, prvkey).await {
    Ok(process) => {
        // Handle process
    }
    Err(e) => {
        if e.conn_err() {
            // Connection error - maybe retry
            eprintln!("Connection error: {}", e);
        } else {
            // Timeout or other error
            // For assign, this is normal - just continue polling
        }
    }
}
```
