//! ColonyOS Rust SDK
//!
//! This library provides a Rust client for interacting with ColonyOS servers.
//!
//! # Example
//!
//! ```rust,no_run
//! use colonyos::{core, submit, assign, close};
//!
//! #[tokio::main]
//! async fn main() {
//!     let prvkey = "your_private_key";
//!     let spec = core::FunctionSpec::new("my_function", "cli", "my_colony");
//!     let process = submit(&spec, prvkey).await.unwrap();
//!     println!("Submitted process: {}", process.processid);
//! }
//! ```

pub mod core;
pub mod crypto;
pub mod rpc;

// Re-export server configuration functions
pub use rpc::{set_server_url, get_server_url};

use serde_json::Value;
use std::collections::HashMap;

// ============== Colony Methods ==============

pub async fn add_colony(
    colony: &core::Colony,
    prvkey: &str,
) -> Result<core::Colony, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_colony_rpcmsg(colony, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let colony: core::Colony = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(colony)
}

pub async fn remove_colony(
    colony_name: &str,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_remove_colony_rpcmsg(&colony_name.to_owned(), &prvkey.to_owned());
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn get_colony(
    colonyname: &str,
    prvkey: &str,
) -> Result<core::Colony, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_colony_rpcmsg(colonyname, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let colony: core::Colony = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(colony)
}

pub async fn get_colonies(
    prvkey: &str,
) -> Result<Vec<core::Colony>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_colonies_rpcmsg(prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let colonies: Result<Vec<core::Colony>, _> = serde_json::from_str(reply_json.as_str());
    Ok(colonies.unwrap_or_default())
}

// ============== Executor Methods ==============

pub async fn add_executor(
    executor: &core::Executor,
    prvkey: &str,
) -> Result<core::Executor, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_executor_rpcmsg(executor, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let executor: core::Executor = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(executor)
}

pub async fn approve_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_approve_executor_rpcmsg(&colonyname.to_owned(), &executorname.to_owned(), &prvkey.to_owned());
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn reject_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_reject_executor_rpcmsg(colonyname, executorname, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn remove_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_remove_executor_rpcmsg(colonyname, executorname, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn get_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<core::Executor, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_executor_rpcmsg(colonyname, executorname, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let executor: core::Executor = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(executor)
}

pub async fn get_executors(
    colonyname: &str,
    prvkey: &str,
) -> Result<Vec<core::Executor>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_executors_rpcmsg(colonyname, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let executors: Result<Vec<core::Executor>, _> = serde_json::from_str(reply_json.as_str());
    Ok(executors.unwrap_or_default())
}

// ============== Process Methods ==============

pub async fn submit(
    spec: &core::FunctionSpec,
    prvkey: &str,
) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_submit_functionspec_rpcmsg(spec, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let process: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(process)
}

pub async fn assign(
    colonyname: &str,
    timeout: i32,
    prvkey: &str,
) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_assign_process_rpcmsg(
        &colonyname.to_owned(),
        timeout,
        &prvkey.to_owned(),
    );
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let process: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(process)
}

pub async fn close(processid: &str, prvkey: &str) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_close_process_rpcmsg(&processid.to_owned(), &prvkey.to_owned());
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn fail(processid: &str, prvkey: &str) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_fail_process_rpcmsg(&processid.to_owned(), &prvkey.to_owned());
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn get_process(processid: &str, prvkey: &str) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_process_rpcmsg(&processid.to_owned(), &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let process: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(process)
}

pub async fn get_processes(
    colonyname: &str,
    count: i32,
    state: i32,
    prvkey: &str,
) -> Result<Vec<core::Process>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_processes_rpcmsg(&colonyname.to_owned(), count, state, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let processes: Result<Vec<core::Process>, _> = serde_json::from_str(reply_json.as_str());
    Ok(processes.unwrap_or_default())
}

pub async fn remove_process(processid: &str, prvkey: &str) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_remove_process_rpcmsg(processid, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn remove_all_processes(
    colonyname: &str,
    state: i32,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_remove_all_processes_rpcmsg(colonyname, state, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn set_output(
    processid: &str,
    output: Vec<String>,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_set_output_rpcmsg(processid, output, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

// ============== Attribute Methods ==============

pub async fn add_attr(
    attr: &core::Attribute,
    prvkey: &str,
) -> Result<core::Attribute, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_attr_rpcmsg(attr, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let attr: core::Attribute = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(attr)
}

// ============== Workflow Methods ==============

pub async fn submit_workflow(
    workflowspec: &core::WorkflowSpec,
    prvkey: &str,
) -> Result<core::ProcessGraph, rpc::RPCError> {
    let rpcmsg = rpc::compose_submit_workflow_rpcmsg(workflowspec, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let processgraph: core::ProcessGraph = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(processgraph)
}

pub async fn get_processgraph(
    processgraphid: &str,
    prvkey: &str,
) -> Result<core::ProcessGraph, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_processgraph_rpcmsg(processgraphid, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let processgraph: core::ProcessGraph = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(processgraph)
}

pub async fn get_processgraphs(
    colonyname: &str,
    count: i32,
    state: i32,
    prvkey: &str,
) -> Result<Vec<core::ProcessGraph>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_processgraphs_rpcmsg(colonyname, count, state, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let processgraphs: Result<Vec<core::ProcessGraph>, _> = serde_json::from_str(reply_json.as_str());
    Ok(processgraphs.unwrap_or_default())
}

pub async fn remove_processgraph(
    processgraphid: &str,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_remove_processgraph_rpcmsg(processgraphid, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn remove_all_processgraphs(
    colonyname: &str,
    state: i32,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_remove_all_processgraphs_rpcmsg(colonyname, state, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

// ============== Log Methods ==============

pub async fn add_log(log: &core::Log, prvkey: &str) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_add_log_rpcmsg(log, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn get_logs(
    colonyname: &str,
    processid: &str,
    executorname: &str,
    count: i32,
    since: i64,
    prvkey: &str,
) -> Result<Vec<core::Log>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_logs_rpcmsg(colonyname, processid, executorname, count, since, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let logs: Result<Vec<core::Log>, _> = serde_json::from_str(reply_json.as_str());
    Ok(logs.unwrap_or_default())
}

// ============== Subscription Methods ==============

/// Subscribe to process lifecycle events and wait for the process to reach the specified state.
///
/// This function opens a WebSocket connection to the server and blocks until the process
/// reaches the desired state (e.g., SUCCESS or FAILED).
///
/// # Arguments
/// * `process` - The process to subscribe to
/// * `state` - The target state to wait for (e.g., core::SUCCESS, core::RUNNING)
/// * `timeout` - Timeout in seconds
/// * `prvkey` - Private key for authentication
///
/// # Example
/// ```rust,no_run
/// use colonyos::{submit, subscribe_process, core};
///
/// async fn wait_for_process() {
///     let prvkey = "your_private_key";
///     let spec = core::FunctionSpec::new("my_function", "cli", "my_colony");
///     let process = submit(&spec, prvkey).await.unwrap();
///
///     // Wait for process to complete
///     subscribe_process(&process, core::SUCCESS, 60, prvkey).await.unwrap();
///     println!("Process completed!");
/// }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub async fn subscribe_process(
    process: &core::Process,
    state: i32,
    timeout: i32,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_subscribe_process_rpcmsg(
        &process.processid,
        &process.spec.conditions.executortype,
        state,
        timeout,
        &process.spec.conditions.colonyname,
        prvkey,
    );
    rpc::send_ws_subscribe_process(rpcmsg).await
}

/// Subscribe to channel messages via WebSocket.
///
/// This function opens a WebSocket connection to receive real-time channel messages.
/// Messages are delivered to the callback function as they arrive.
///
/// # Arguments
/// * `processid` - The process ID that owns the channel
/// * `channelname` - Name of the channel
/// * `afterseq` - Start reading after this sequence number (0 for all messages)
/// * `timeout` - Timeout in seconds
/// * `prvkey` - Private key for authentication
/// * `callback` - Function called for each batch of messages. Return false to stop receiving.
///
/// # Returns
/// All received messages collected during the subscription.
///
/// # Example
/// ```rust,no_run
/// use colonyos::{subscribe_channel, core};
///
/// async fn receive_messages(processid: &str, prvkey: &str) {
///     let entries = subscribe_channel(
///         processid,
///         "my-channel",
///         0,  // start from beginning
///         30, // 30 second timeout
///         prvkey,
///         |entries| {
///             for entry in &entries {
///                 println!("Received: {}", entry.payload_as_string());
///             }
///             true // continue receiving
///         }
///     ).await.unwrap();
/// }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub async fn subscribe_channel<F>(
    processid: &str,
    channelname: &str,
    afterseq: i64,
    timeout: i32,
    prvkey: &str,
    callback: F,
) -> Result<Vec<core::ChannelEntry>, rpc::RPCError>
where
    F: FnMut(Vec<core::ChannelEntry>) -> bool,
{
    let rpcmsg = rpc::compose_subscribe_channel_rpcmsg(
        processid,
        channelname,
        afterseq,
        timeout,
        prvkey,
    );
    rpc::send_ws_subscribe_channel(rpcmsg, timeout, callback).await
}

// ============== Channel Methods ==============

pub async fn channel_append(
    processid: &str,
    channelname: &str,
    sequence: i64,
    data: &str,
    data_type: &str,
    inreplyto: i64,
    prvkey: &str,
) -> Result<core::ChannelEntry, rpc::RPCError> {
    let rpcmsg = rpc::compose_channel_append_rpcmsg(processid, channelname, sequence, data, data_type, inreplyto, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let entry: core::ChannelEntry = serde_json::from_str(reply_json.as_str()).unwrap_or_default();
    Ok(entry)
}

pub async fn channel_read(
    processid: &str,
    channelname: &str,
    start: i64,
    count: i32,
    prvkey: &str,
) -> Result<Vec<core::ChannelEntry>, rpc::RPCError> {
    let rpcmsg = rpc::compose_channel_read_rpcmsg(processid, channelname, start, count, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let entries: Result<Vec<core::ChannelEntry>, _> = serde_json::from_str(reply_json.as_str());
    Ok(entries.unwrap_or_default())
}

// ============== Statistics Methods ==============

pub async fn get_statistics(
    colonyname: &str,
    prvkey: &str,
) -> Result<core::Statistics, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_statistics_rpcmsg(colonyname, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let stats: core::Statistics = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(stats)
}

// ============== Function Registry Methods ==============

pub async fn add_function(
    function: &core::Function,
    prvkey: &str,
) -> Result<core::Function, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_function_rpcmsg(function, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let function: core::Function = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(function)
}

pub async fn get_functions(
    colonyname: &str,
    prvkey: &str,
) -> Result<Vec<core::Function>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_functions_rpcmsg(colonyname, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let functions: Result<Vec<core::Function>, _> = serde_json::from_str(reply_json.as_str());
    Ok(functions.unwrap_or_default())
}

pub async fn get_functions_by_executor(
    colonyname: &str,
    executorname: &str,
    prvkey: &str,
) -> Result<Vec<core::Function>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_functions_by_executor_rpcmsg(colonyname, executorname, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let functions: Result<Vec<core::Function>, _> = serde_json::from_str(reply_json.as_str());
    Ok(functions.unwrap_or_default())
}

pub async fn remove_function(
    functionid: &str,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_remove_function_rpcmsg(functionid, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

// ============== Blueprint Definition Methods ==============

pub async fn add_blueprint_definition(
    definition: &core::BlueprintDefinition,
    prvkey: &str,
) -> Result<core::BlueprintDefinition, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_blueprint_definition_rpcmsg(definition, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let definition: core::BlueprintDefinition = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(definition)
}

pub async fn get_blueprint_definition(
    colonyname: &str,
    name: &str,
    prvkey: &str,
) -> Result<core::BlueprintDefinition, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_blueprint_definition_rpcmsg(colonyname, name, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let definition: core::BlueprintDefinition = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(definition)
}

pub async fn get_blueprint_definitions(
    colonyname: &str,
    prvkey: &str,
) -> Result<Vec<core::BlueprintDefinition>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_blueprint_definitions_rpcmsg(colonyname, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let definitions: Result<Vec<core::BlueprintDefinition>, _> = serde_json::from_str(reply_json.as_str());
    Ok(definitions.unwrap_or_default())
}

pub async fn remove_blueprint_definition(
    colonyname: &str,
    name: &str,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_remove_blueprint_definition_rpcmsg(colonyname, name, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

// ============== Blueprint Methods ==============

pub async fn add_blueprint(
    blueprint: &core::Blueprint,
    prvkey: &str,
) -> Result<core::Blueprint, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_blueprint_rpcmsg(blueprint, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let blueprint: core::Blueprint = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(blueprint)
}

pub async fn get_blueprint(
    colonyname: &str,
    name: &str,
    prvkey: &str,
) -> Result<core::Blueprint, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_blueprint_rpcmsg(colonyname, name, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let blueprint: core::Blueprint = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(blueprint)
}

pub async fn get_blueprints(
    colonyname: &str,
    kind: &str,
    location: &str,
    prvkey: &str,
) -> Result<Vec<core::Blueprint>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_blueprints_rpcmsg(colonyname, kind, location, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let blueprints: Result<Vec<core::Blueprint>, _> = serde_json::from_str(reply_json.as_str());
    Ok(blueprints.unwrap_or_default())
}

pub async fn update_blueprint(
    blueprint: &core::Blueprint,
    force_generation: bool,
    prvkey: &str,
) -> Result<core::Blueprint, rpc::RPCError> {
    let rpcmsg = rpc::compose_update_blueprint_rpcmsg(blueprint, force_generation, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let blueprint: core::Blueprint = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(blueprint)
}

pub async fn remove_blueprint(
    colonyname: &str,
    name: &str,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_remove_blueprint_rpcmsg(colonyname, name, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn update_blueprint_status(
    colonyname: &str,
    name: &str,
    status: HashMap<String, Value>,
    prvkey: &str,
) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_update_blueprint_status_rpcmsg(colonyname, name, status, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;
    Ok(())
}

pub async fn reconcile_blueprint(
    colonyname: &str,
    name: &str,
    force: bool,
    prvkey: &str,
) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_reconcile_blueprint_rpcmsg(colonyname, name, force, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let process: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(process)
}

// ============== Unit Tests ==============

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::*;

    #[test]
    fn test_server_url_configuration() {
        // Test default URL
        let default_url = get_server_url();
        assert!(default_url.contains("localhost") || default_url.contains("50080"));

        // Test setting a custom URL
        set_server_url("http://custom-server:8080/api");
        let custom_url = get_server_url();
        assert_eq!(custom_url, "http://custom-server:8080/api");

        // Reset to default for other tests
        set_server_url("http://localhost:50080/api");
    }

    #[test]
    fn test_module_exports() {
        // Verify core module is accessible
        let _colony = core::Colony::new("id", "name");
        let _executor = core::Executor::new("name", "id", "cli", "colony");
        let _spec = core::FunctionSpec::new("func", "cli", "colony");
        let _attr = core::Attribute::new("colony", "id", "key", "value");

        // Verify crypto module is accessible
        let prvkey = crypto::gen_prvkey();
        assert_eq!(prvkey.len(), 64);

        let id = crypto::gen_id(&prvkey);
        assert_eq!(id.len(), 64);
    }

    #[test]
    fn test_crypto_exports() {
        // Test all crypto functions are accessible
        let key = crypto::gen_prvkey();
        let id = crypto::gen_id(&key);
        let hash = crypto::gen_hash("test");
        let sig = crypto::gen_signature("test", &key);
        let recovered = crypto::recid("test", &sig);

        assert_eq!(id, recovered);
        assert_eq!(hash.len(), 64);
        assert_eq!(sig.len(), 130);
    }

    #[test]
    fn test_core_state_constants_accessible() {
        // Verify state constants are accessible
        assert_eq!(core::WAITING, 0);
        assert_eq!(core::RUNNING, 1);
        assert_eq!(core::SUCCESS, 2);
        assert_eq!(core::FAILED, 3);

        assert_eq!(core::PENDING, 0);
        assert_eq!(core::APPROVED, 1);
        assert_eq!(core::REJECTED, 2);

        assert_eq!(core::IN, 0);
        assert_eq!(core::OUT, 1);
        assert_eq!(core::ERR, 2);
        assert_eq!(core::ENV, 4);
    }

    #[test]
    fn test_colony_creation() {
        let colony = Colony::new("test-id", "test-colony");
        assert_eq!(colony.colonyid, "test-id");
        assert_eq!(colony.name, "test-colony");
    }

    #[test]
    fn test_executor_creation() {
        let executor = Executor::new("test-executor", "exec-id", "cli", "test-colony");
        assert_eq!(executor.executorname, "test-executor");
        assert_eq!(executor.executorid, "exec-id");
        assert_eq!(executor.executortype, "cli");
        assert_eq!(executor.colonyname, "test-colony");
        assert_eq!(executor.state, 0);
    }

    #[test]
    fn test_functionspec_creation() {
        let spec = FunctionSpec::new("my-func", "cli", "my-colony");
        assert_eq!(spec.funcname, "my-func");
        assert_eq!(spec.conditions.executortype, "cli");
        assert_eq!(spec.conditions.colonyname, "my-colony");
        assert!(spec.args.is_empty());
        assert!(spec.kwargs.is_empty());
    }

    #[test]
    fn test_conditions_creation() {
        let conditions = Conditions::new("test-colony", "docker");
        assert_eq!(conditions.colonyname, "test-colony");
        assert_eq!(conditions.executortype, "docker");
        assert!(conditions.executornames.is_empty());
        assert!(conditions.dependencies.is_empty());
    }

    #[test]
    fn test_attribute_creation() {
        let attr = Attribute::new("test-colony", "process-123", "result", "success");
        assert_eq!(attr.targetcolonyname, "test-colony");
        assert_eq!(attr.targetid, "process-123");
        assert_eq!(attr.key, "result");
        assert_eq!(attr.value, "success");
        assert_eq!(attr.attributetype, OUT);
    }

    #[test]
    fn test_state_constants() {
        assert_eq!(WAITING, 0);
        assert_eq!(RUNNING, 1);
        assert_eq!(SUCCESS, 2);
        assert_eq!(FAILED, 3);
    }

    #[test]
    fn test_executor_state_constants() {
        assert_eq!(PENDING, 0);
        assert_eq!(APPROVED, 1);
        assert_eq!(REJECTED, 2);
    }

    #[test]
    fn test_attribute_type_constants() {
        assert_eq!(IN, 0);
        assert_eq!(OUT, 1);
        assert_eq!(ERR, 2);
        assert_eq!(ENV, 4);
    }

    #[test]
    fn test_functionspec_serialization() {
        let mut spec = FunctionSpec::new("test-func", "cli", "test-colony");
        spec.args = vec!["arg1".to_string(), "arg2".to_string()];
        spec.priority = 5;
        spec.maxwaittime = 100;
        spec.maxexectime = 300;
        spec.maxretries = 3;

        let json = serde_json::to_string(&spec).unwrap();
        let parsed: FunctionSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.funcname, "test-func");
        assert_eq!(parsed.args.len(), 2);
        assert_eq!(parsed.priority, 5);
        assert_eq!(parsed.maxwaittime, 100);
        assert_eq!(parsed.maxexectime, 300);
        assert_eq!(parsed.maxretries, 3);
    }

    #[test]
    fn test_process_deserialization() {
        let json = r#"{
            "processid": "proc-123",
            "state": 1,
            "isassigned": true,
            "spec": {
                "funcname": "test",
                "conditions": {
                    "colonyname": "test-colony"
                }
            }
        }"#;

        let process: Process = serde_json::from_str(json).unwrap();
        assert_eq!(process.processid, "proc-123");
        assert_eq!(process.state, RUNNING);
        assert!(process.isassigned);
    }

    #[test]
    fn test_processgraph_deserialization() {
        let json = r#"{
            "processgraphid": "pg-123",
            "colonyname": "test-colony",
            "state": 0,
            "rootprocessids": ["proc-1", "proc-2"],
            "processids": ["proc-1", "proc-2", "proc-3"]
        }"#;

        let pg: ProcessGraph = serde_json::from_str(json).unwrap();
        assert_eq!(pg.processgraphid, "pg-123");
        assert_eq!(pg.colonyname, "test-colony");
        assert_eq!(pg.rootprocessids.len(), 2);
        assert_eq!(pg.processids.len(), 3);
    }

    #[test]
    fn test_log_serialization() {
        let log = Log {
            processid: "proc-123".to_string(),
            colonyname: "test-colony".to_string(),
            executorname: "executor-1".to_string(),
            message: "Test message".to_string(),
            timestamp: 1234567890,
        };

        let json = serde_json::to_string(&log).unwrap();
        let parsed: Log = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.processid, "proc-123");
        assert_eq!(parsed.message, "Test message");
        assert_eq!(parsed.timestamp, 1234567890);
    }

    #[test]
    fn test_channel_entry_deserialization() {
        // Server returns payload as base64 encoded string
        let json = r#"{
            "sequence": 42,
            "payload": "aGVsbG8=",
            "type": "data",
            "inreplyto": 0,
            "timestamp": "2025-01-01T00:00:00Z",
            "senderid": "abc123"
        }"#;

        let entry: ChannelEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.sequence, 42);
        assert_eq!(entry.payload_as_string(), "hello");
        assert_eq!(entry.msgtype, "data");
        assert_eq!(entry.timestamp, "2025-01-01T00:00:00Z");
        assert_eq!(entry.senderid, "abc123");
    }

    #[test]
    fn test_statistics_default() {
        let stats = Statistics::default();
        assert_eq!(stats.colonies, 0);
        assert_eq!(stats.executors, 0);
        assert_eq!(stats.waitingprocesses, 0);
        assert_eq!(stats.runningprocesses, 0);
    }

    #[test]
    fn test_blueprint_deserialization() {
        let json = r#"{
            "blueprintid": "bp-123",
            "kind": "Deployment",
            "metadata": {
                "name": "my-app",
                "colonyname": "test-colony"
            },
            "handler": {
                "executortype": "docker-reconciler"
            },
            "spec": {},
            "status": {},
            "generation": 1,
            "reconciledgeneration": 0
        }"#;

        let blueprint: Blueprint = serde_json::from_str(json).unwrap();
        assert_eq!(blueprint.blueprintid, "bp-123");
        assert_eq!(blueprint.kind, "Deployment");
        assert_eq!(blueprint.metadata.name, "my-app");
        assert_eq!(blueprint.handler.executortype, "docker-reconciler");
        assert_eq!(blueprint.generation, 1);
    }

    #[test]
    fn test_function_deserialization() {
        let json = r#"{
            "functionid": "func-123",
            "executorname": "executor-1",
            "executortype": "cli",
            "colonyname": "test-colony",
            "funcname": "process_data",
            "counter": 100,
            "avgexectime": 1.5
        }"#;

        let function: Function = serde_json::from_str(json).unwrap();
        assert_eq!(function.functionid, "func-123");
        assert_eq!(function.funcname, "process_data");
        assert_eq!(function.counter, 100);
        assert_eq!(function.avgexectime, 1.5);
    }

    #[test]
    fn test_workflowspec_serialization() {
        let spec1 = FunctionSpec::new("step1", "cli", "test-colony");
        let spec2 = FunctionSpec::new("step2", "cli", "test-colony");

        let workflow = WorkflowSpec {
            colonyname: "test-colony".to_string(),
            functionspecs: vec![spec1, spec2],
        };

        let json = serde_json::to_string(&workflow).unwrap();
        let parsed: WorkflowSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.colonyname, "test-colony");
        assert_eq!(parsed.functionspecs.len(), 2);
        assert_eq!(parsed.functionspecs[0].funcname, "step1");
        assert_eq!(parsed.functionspecs[1].funcname, "step2");
    }

    #[test]
    fn test_gpu_default() {
        let gpu = GPU::default();
        assert_eq!(gpu.name, "");
        assert_eq!(gpu.mem, "");
        assert_eq!(gpu.count, 0);
        assert_eq!(gpu.nodecount, 0);
    }

    #[test]
    fn test_filesystem_default() {
        let fs = Filesystem::default();
        assert_eq!(fs.mount, "");
        assert!(fs.snapshots.is_empty());
        assert!(fs.dirs.is_empty());
    }

    #[test]
    fn test_capabilities_is_empty() {
        let caps = Capabilities::default();
        assert!(caps.is_empty());

        let mut caps_with_hw = Capabilities::default();
        caps_with_hw.hardware.push(Hardware::default());
        assert!(!caps_with_hw.is_empty());

        let mut caps_with_sw = Capabilities::default();
        caps_with_sw.software.push(Software::default());
        assert!(!caps_with_sw.is_empty());
    }

    #[test]
    fn test_allocations_is_empty() {
        let allocs = Allocations::default();
        assert!(allocs.is_empty());

        let mut allocs_with_project = Allocations::default();
        allocs_with_project.projects.insert("test".to_string(), Project::default());
        assert!(!allocs_with_project.is_empty());
    }

    #[test]
    fn test_hardware_default() {
        let hw = Hardware::default();
        assert_eq!(hw.model, "");
        assert_eq!(hw.nodes, 0);
        assert_eq!(hw.cpu, "");
        assert_eq!(hw.cores, 0);
        assert_eq!(hw.mem, "");
        assert_eq!(hw.storage, "");
        assert_eq!(hw.platform, "");
        assert_eq!(hw.architecture, "");
        assert!(hw.network.is_empty());
    }

    #[test]
    fn test_software_default() {
        let sw = Software::default();
        assert_eq!(sw.name, "");
        assert_eq!(sw.software_type, "");
        assert_eq!(sw.version, "");
    }

    #[test]
    fn test_project_default() {
        let proj = Project::default();
        assert_eq!(proj.allocatedcpu, 0);
        assert_eq!(proj.usedcpu, 0);
        assert_eq!(proj.allocatedgpu, 0);
        assert_eq!(proj.usedgpu, 0);
        assert_eq!(proj.allocatedstorage, 0);
        assert_eq!(proj.usedstorage, 0);
    }

    #[test]
    fn test_executor_serialization_skips_empty_fields() {
        let executor = Executor::new("test", "id", "cli", "colony");
        let json = serde_json::to_string(&executor).unwrap();

        // Empty fields should not be serialized
        assert!(!json.contains("\"commissiontime\""));
        assert!(!json.contains("\"lastheardfromtime\""));
        assert!(!json.contains("\"locationname\""));
        assert!(!json.contains("\"blueprintid\""));

        // Required fields should be present
        assert!(json.contains("\"executorid\""));
        assert!(json.contains("\"executorname\""));
        assert!(json.contains("\"executortype\""));
        assert!(json.contains("\"colonyname\""));
    }

    #[test]
    fn test_process_with_null_fields() {
        // Test that null values are handled correctly
        let json = r#"{
            "processid": "proc-123",
            "initiatorid": null,
            "initiatorname": null,
            "assignedexecutorid": null,
            "isassigned": false,
            "state": 0,
            "attributes": null,
            "spec": {
                "funcname": "test",
                "args": null,
                "kwargs": null,
                "conditions": {
                    "colonyname": "test",
                    "executornames": null,
                    "dependencies": null
                },
                "env": null,
                "channels": null
            },
            "parents": null,
            "children": null,
            "input": null,
            "output": null,
            "errors": null
        }"#;

        let process: Process = serde_json::from_str(json).unwrap();
        assert_eq!(process.processid, "proc-123");
        assert_eq!(process.initiatorid, "");
        assert!(process.attributes.is_empty());
        assert!(process.spec.args.is_empty());
        assert!(process.parents.is_empty());
        assert!(process.output.is_empty());
    }

    #[test]
    fn test_executor_with_null_capabilities() {
        let json = r#"{
            "executorid": "id-123",
            "executorname": "test",
            "executortype": "cli",
            "colonyname": "colony",
            "state": 0,
            "capabilities": null,
            "allocations": null
        }"#;

        let executor: Executor = serde_json::from_str(json).unwrap();
        assert_eq!(executor.executorid, "id-123");
        assert!(executor.capabilities.is_empty());
        assert!(executor.allocations.is_empty());
    }

    #[test]
    fn test_blueprint_definition_default() {
        let def = BlueprintDefinition::default();
        assert_eq!(def.blueprintdefinitionid, "");
        assert_eq!(def.kind, "");
        assert_eq!(def.metadata.name, "");
        assert_eq!(def.metadata.colonyname, "");
        assert_eq!(def.spec.names.kind, "");
        assert_eq!(def.spec.handler.executor_type, "");
    }

    #[test]
    fn test_blueprint_metadata_default() {
        let meta = BlueprintMetadata::default();
        assert_eq!(meta.name, "");
        assert_eq!(meta.colonyname, "");
    }

    #[test]
    fn test_blueprint_handler_default() {
        let handler = BlueprintHandler::default();
        assert_eq!(handler.executortype, "");
    }

    #[test]
    fn test_blueprint_default() {
        let bp = Blueprint::default();
        assert_eq!(bp.blueprintid, "");
        assert_eq!(bp.kind, "");
        assert_eq!(bp.generation, 0);
        assert_eq!(bp.reconciledgeneration, 0);
        assert!(bp.spec.is_empty());
        assert!(bp.status.is_empty());
    }

    #[test]
    fn test_channel_entry_with_null_fields() {
        let json = r#"{
            "sequence": 1,
            "payload": null,
            "type": null,
            "inreplyto": 0,
            "timestamp": null,
            "senderid": null
        }"#;

        let entry: ChannelEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.sequence, 1);
        assert!(entry.payload.is_empty());
        assert_eq!(entry.msgtype, "");
        assert_eq!(entry.timestamp, "");
        assert_eq!(entry.senderid, "");
    }

    #[test]
    fn test_log_with_null_fields() {
        let json = r#"{
            "processid": null,
            "colonyname": null,
            "executorname": null,
            "message": null,
            "timestamp": 0
        }"#;

        let log: Log = serde_json::from_str(json).unwrap();
        assert_eq!(log.processid, "");
        assert_eq!(log.colonyname, "");
        assert_eq!(log.executorname, "");
        assert_eq!(log.message, "");
    }

    #[test]
    fn test_functionspec_with_env_and_kwargs() {
        let mut spec = FunctionSpec::new("func", "cli", "colony");
        spec.env.insert("KEY1".to_string(), "value1".to_string());
        spec.env.insert("KEY2".to_string(), "value2".to_string());
        spec.kwargs.insert("param1".to_string(), serde_json::json!(123));
        spec.kwargs.insert("param2".to_string(), serde_json::json!("string"));

        let json = serde_json::to_string(&spec).unwrap();
        let parsed: FunctionSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.env.get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(parsed.env.get("KEY2"), Some(&"value2".to_string()));
        assert_eq!(parsed.kwargs.get("param1"), Some(&serde_json::json!(123)));
    }

    #[test]
    fn test_conditions_with_dependencies() {
        let mut conditions = Conditions::new("colony", "cli");
        conditions.dependencies = vec!["step1".to_string(), "step2".to_string()];
        conditions.executornames = vec!["executor1".to_string()];
        conditions.nodes = 2;
        conditions.walltime = 3600;

        let json = serde_json::to_string(&conditions).unwrap();
        let parsed: Conditions = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.dependencies.len(), 2);
        assert_eq!(parsed.executornames.len(), 1);
        assert_eq!(parsed.nodes, 2);
        assert_eq!(parsed.walltime, 3600);
    }

    #[test]
    fn test_gpu_with_values() {
        let json = r#"{
            "name": "NVIDIA A100",
            "mem": "40GB",
            "count": 4,
            "nodecount": 2
        }"#;

        let gpu: GPU = serde_json::from_str(json).unwrap();
        assert_eq!(gpu.name, "NVIDIA A100");
        assert_eq!(gpu.mem, "40GB");
        assert_eq!(gpu.count, 4);
        assert_eq!(gpu.nodecount, 2);
    }

    #[test]
    fn test_statistics_serialization() {
        let stats = Statistics {
            colonies: 5,
            executors: 10,
            waitingprocesses: 100,
            runningprocesses: 50,
            successfulprocesses: 1000,
            failedprocesses: 10,
            waitingworkflows: 5,
            runningworkflows: 2,
            successfulworkflows: 50,
            failedworkflows: 1,
        };

        let json = serde_json::to_string(&stats).unwrap();
        let parsed: Statistics = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.colonies, 5);
        assert_eq!(parsed.executors, 10);
        assert_eq!(parsed.waitingprocesses, 100);
        assert_eq!(parsed.successfulprocesses, 1000);
    }

    #[test]
    fn test_failure_deserialization() {
        let json = r#"{
            "status": 500,
            "message": "Internal server error"
        }"#;

        let failure: Failure = serde_json::from_str(json).unwrap();
        assert_eq!(failure.status, 500);
        assert_eq!(failure.message, "Internal server error");
    }

    #[test]
    fn test_processgraph_with_null_fields() {
        let json = r#"{
            "processgraphid": null,
            "colonyname": null,
            "state": 0,
            "rootprocessids": null,
            "processids": null
        }"#;

        let pg: ProcessGraph = serde_json::from_str(json).unwrap();
        assert_eq!(pg.processgraphid, "");
        assert_eq!(pg.colonyname, "");
        assert!(pg.rootprocessids.is_empty());
        assert!(pg.processids.is_empty());
    }

    #[test]
    fn test_workflowspec_with_null_fields() {
        let json = r#"{
            "colonyname": null,
            "functionspecs": null
        }"#;

        let ws: WorkflowSpec = serde_json::from_str(json).unwrap();
        assert_eq!(ws.colonyname, "");
        assert!(ws.functionspecs.is_empty());
    }

    #[test]
    fn test_channel_entry_payload_bytes() {
        // Test base64 encoded payload
        let entry = ChannelEntry {
            sequence: 1,
            payload: "SGVsbG8gV29ybGQ=".to_string(), // "Hello World" in base64
            msgtype: "data".to_string(),
            inreplyto: 0,
            timestamp: "".to_string(),
            senderid: "".to_string(),
        };
        let bytes = entry.payload_bytes();
        assert_eq!(bytes, b"Hello World");
    }

    #[test]
    fn test_channel_entry_payload_bytes_empty() {
        let entry = ChannelEntry::default();
        let bytes = entry.payload_bytes();
        assert!(bytes.is_empty());
    }

    #[test]
    fn test_channel_entry_invalid_base64() {
        let entry = ChannelEntry {
            sequence: 1,
            payload: "not valid base64!!!".to_string(),
            msgtype: "data".to_string(),
            inreplyto: 0,
            timestamp: "".to_string(),
            senderid: "".to_string(),
        };
        // Should return empty vec on invalid base64
        let bytes = entry.payload_bytes();
        assert!(bytes.is_empty());
    }

    #[test]
    fn test_channel_entry_default() {
        let entry = ChannelEntry::default();
        assert_eq!(entry.sequence, 0);
        assert_eq!(entry.payload, "");
        assert_eq!(entry.msgtype, "");
        assert_eq!(entry.inreplyto, 0);
        assert_eq!(entry.timestamp, "");
        assert_eq!(entry.senderid, "");
    }

    #[test]
    fn test_attribute_types() {
        let attr_in = Attribute {
            attributetype: IN,
            ..Attribute::new("colony", "process", "key", "value")
        };
        assert_eq!(attr_in.attributetype, 0);

        let attr_err = Attribute {
            attributetype: ERR,
            ..Attribute::new("colony", "process", "key", "error")
        };
        assert_eq!(attr_err.attributetype, 2);

        let attr_env = Attribute {
            attributetype: ENV,
            ..Attribute::new("colony", "process", "key", "env_val")
        };
        assert_eq!(attr_env.attributetype, 4);
    }

    #[test]
    fn test_executor_states() {
        let mut executor = Executor::new("name", "id", "cli", "colony");
        executor.state = PENDING;
        assert_eq!(executor.state, 0);

        executor.state = APPROVED;
        assert_eq!(executor.state, 1);

        executor.state = REJECTED;
        assert_eq!(executor.state, 2);
    }

    #[test]
    fn test_process_states() {
        let process_json_waiting = r#"{"processid": "p1", "state": 0, "spec": {"funcname": "test", "conditions": {}}}"#;
        let p: Process = serde_json::from_str(process_json_waiting).unwrap();
        assert_eq!(p.state, WAITING);

        let process_json_success = r#"{"processid": "p2", "state": 2, "spec": {"funcname": "test", "conditions": {}}}"#;
        let p: Process = serde_json::from_str(process_json_success).unwrap();
        assert_eq!(p.state, SUCCESS);

        let process_json_failed = r#"{"processid": "p3", "state": 3, "spec": {"funcname": "test", "conditions": {}}}"#;
        let p: Process = serde_json::from_str(process_json_failed).unwrap();
        assert_eq!(p.state, FAILED);
    }

    #[test]
    fn test_functionspec_with_channels() {
        let mut spec = FunctionSpec::new("func", "cli", "colony");
        spec.channels.push("input".to_string());
        spec.channels.push("output".to_string());

        let json = serde_json::to_string(&spec).unwrap();
        let parsed: FunctionSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.channels.len(), 2);
        assert_eq!(parsed.channels[0], "input");
        assert_eq!(parsed.channels[1], "output");
    }

    #[test]
    fn test_functionspec_with_nodename() {
        let mut spec = FunctionSpec::new("func", "cli", "colony");
        spec.nodename = "step1".to_string();
        spec.label = "Step One".to_string();

        let json = serde_json::to_string(&spec).unwrap();
        let parsed: FunctionSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.nodename, "step1");
        assert_eq!(parsed.label, "Step One");
    }

    #[test]
    fn test_conditions_with_gpu() {
        let mut conditions = Conditions::new("colony", "cli");
        conditions.gpu.name = "NVIDIA".to_string();
        conditions.gpu.count = 2;
        conditions.gpu.mem = "16GB".to_string();

        let json = serde_json::to_string(&conditions).unwrap();
        let parsed: Conditions = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.gpu.name, "NVIDIA");
        assert_eq!(parsed.gpu.count, 2);
        assert_eq!(parsed.gpu.mem, "16GB");
    }

    #[test]
    fn test_process_with_output_and_errors() {
        // Note: Process uses "in" and "out" for JSON field names
        let json = r#"{
            "processid": "proc-123",
            "state": 3,
            "spec": {"funcname": "test", "conditions": {}},
            "out": ["result1", "result2"],
            "errors": ["error1"]
        }"#;

        let process: Process = serde_json::from_str(json).unwrap();
        assert_eq!(process.output.len(), 2);
        assert_eq!(process.output[0], "result1");
        assert_eq!(process.errors.len(), 1);
        assert_eq!(process.errors[0], "error1");
    }

    #[test]
    fn test_process_with_input() {
        // Note: Process uses "in" for input field in JSON
        let json = r#"{
            "processid": "proc-123",
            "state": 0,
            "spec": {"funcname": "test", "conditions": {}},
            "in": ["input1", "input2", "input3"]
        }"#;

        let process: Process = serde_json::from_str(json).unwrap();
        assert_eq!(process.input.len(), 3);
        assert_eq!(process.input[0], "input1");
    }

    #[test]
    fn test_process_with_parents_and_children() {
        let json = r#"{
            "processid": "proc-123",
            "state": 0,
            "spec": {"funcname": "test", "conditions": {}},
            "parents": ["parent1", "parent2"],
            "children": ["child1"]
        }"#;

        let process: Process = serde_json::from_str(json).unwrap();
        assert_eq!(process.parents.len(), 2);
        assert_eq!(process.children.len(), 1);
    }

    #[test]
    fn test_process_with_attributes() {
        let json = r#"{
            "processid": "proc-123",
            "state": 0,
            "spec": {"funcname": "test", "conditions": {}},
            "attributes": [
                {"key": "key1", "value": "value1", "attributetype": 0, "targetid": "proc-123", "targetcolonyname": "colony"},
                {"key": "key2", "value": "value2", "attributetype": 1, "targetid": "proc-123", "targetcolonyname": "colony"}
            ]
        }"#;

        let process: Process = serde_json::from_str(json).unwrap();
        assert_eq!(process.attributes.len(), 2);
        assert_eq!(process.attributes[0].key, "key1");
        assert_eq!(process.attributes[1].attributetype, OUT);
    }

    #[test]
    fn test_hardware_with_network() {
        let json = r#"{
            "model": "SuperComputer",
            "nodes": 100,
            "cpu": "AMD EPYC",
            "cores": 128,
            "mem": "1TB",
            "storage": "100PB",
            "platform": "HPC",
            "architecture": "x86_64",
            "network": ["InfiniBand", "Ethernet"]
        }"#;

        let hw: Hardware = serde_json::from_str(json).unwrap();
        assert_eq!(hw.network.len(), 2);
        assert_eq!(hw.network[0], "InfiniBand");
        assert_eq!(hw.cores, 128);
    }

    #[test]
    fn test_capabilities_with_hardware_and_software() {
        let mut caps = Capabilities::default();
        assert!(caps.is_empty());

        caps.hardware.push(Hardware {
            model: "Server".to_string(),
            nodes: 1,
            cpu: "Intel".to_string(),
            cores: 64,
            mem: "256GB".to_string(),
            storage: "10TB".to_string(),
            platform: "Cloud".to_string(),
            architecture: "x86_64".to_string(),
            network: vec!["Ethernet".to_string()],
            gpu: GPU::default(),
        });

        caps.software.push(Software {
            name: "Python".to_string(),
            software_type: "Runtime".to_string(),
            version: "3.11".to_string(),
        });

        assert!(!caps.is_empty());
        assert_eq!(caps.hardware.len(), 1);
        assert_eq!(caps.software.len(), 1);
    }

    #[test]
    fn test_filesystem_with_dirs_and_snapshots() {
        let json = r#"{
            "mount": "/data",
            "dirs": [
                {"label": "input", "dir": "/data/input", "keepfiles": true, "onconflicts": {"onstart": {"keeplocal": true}, "onclose": {"keeplocal": false}}},
                {"label": "output", "dir": "/data/output", "keepfiles": false, "onconflicts": {"onstart": {"keeplocal": false}, "onclose": {"keeplocal": true}}}
            ],
            "snapshots": [
                {"snapshotid": "snap1", "label": "snapshot1", "dir": "/snap1", "keepfiles": true, "keepsnapshot": true},
                {"snapshotid": "snap2", "label": "snapshot2", "dir": "/snap2", "keepfiles": false, "keepsnapshot": false}
            ]
        }"#;

        let fs: Filesystem = serde_json::from_str(json).unwrap();
        assert_eq!(fs.mount, "/data");
        assert_eq!(fs.dirs.len(), 2);
        assert_eq!(fs.snapshots.len(), 2);
        assert_eq!(fs.dirs[0].label, "input");
        assert_eq!(fs.snapshots[0].snapshotid, "snap1");
    }

    #[test]
    fn test_blueprint_with_full_spec() {
        let json = r#"{
            "blueprintid": "bp-123",
            "kind": "Deployment",
            "metadata": {
                "name": "my-app",
                "colonyname": "production"
            },
            "handler": {
                "executortype": "docker-reconciler"
            },
            "spec": {
                "replicas": 3,
                "image": "myapp:latest"
            },
            "status": {
                "ready": true,
                "availableReplicas": 3
            },
            "generation": 5,
            "reconciledgeneration": 4
        }"#;

        let bp: Blueprint = serde_json::from_str(json).unwrap();
        assert_eq!(bp.generation, 5);
        assert_eq!(bp.reconciledgeneration, 4);
        assert_eq!(bp.spec.get("replicas"), Some(&serde_json::json!(3)));
        assert_eq!(bp.status.get("ready"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn test_blueprint_definition_with_schemas() {
        let json = r#"{
            "kind": "Deployment",
            "metadata": {
                "name": "deployment-def",
                "colonyname": "production"
            },
            "spec": {
                "names": {
                    "kind": "Deployment",
                    "singular": "deployment",
                    "plural": "deployments"
                },
                "handler": {
                    "executorType": "docker-reconciler"
                },
                "schema": {
                    "type": "object",
                    "properties": {"replicas": {"type": "number"}}
                }
            }
        }"#;

        let def: BlueprintDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(def.metadata.name, "deployment-def");
        assert_eq!(def.spec.names.kind, "Deployment");
        assert_eq!(def.spec.handler.executor_type, "docker-reconciler");
        assert!(def.spec.schema.is_some());
    }

    #[test]
    fn test_allocations_with_multiple_projects() {
        let mut allocs = Allocations::default();
        assert!(allocs.is_empty());

        allocs.projects.insert("project1".to_string(), Project {
            allocatedcpu: 100,
            usedcpu: 50,
            allocatedgpu: 10,
            usedgpu: 5,
            allocatedstorage: 1000,
            usedstorage: 500,
        });

        allocs.projects.insert("project2".to_string(), Project {
            allocatedcpu: 200,
            usedcpu: 100,
            allocatedgpu: 20,
            usedgpu: 10,
            allocatedstorage: 2000,
            usedstorage: 1000,
        });

        assert!(!allocs.is_empty());
        assert_eq!(allocs.projects.len(), 2);
        assert_eq!(allocs.projects.get("project1").unwrap().allocatedcpu, 100);
    }

    #[test]
    fn test_executor_with_full_details() {
        let json = r#"{
            "executorid": "exec-123",
            "executorname": "worker-1",
            "executortype": "docker",
            "colonyname": "production",
            "state": 1,
            "commissiontime": "2025-01-01T00:00:00Z",
            "lastheardfromtime": "2025-01-01T12:00:00Z",
            "locationname": "us-west-2",
            "blueprintid": "bp-456"
        }"#;

        let executor: Executor = serde_json::from_str(json).unwrap();
        assert_eq!(executor.state, APPROVED);
        assert_eq!(executor.locationname, "us-west-2");
        assert_eq!(executor.blueprintid, "bp-456");
        assert_eq!(executor.commissiontime, "2025-01-01T00:00:00Z");
    }

    #[test]
    fn test_function_with_timing_stats() {
        let json = r#"{
            "functionid": "func-123",
            "executorname": "worker",
            "executortype": "cli",
            "colonyname": "colony",
            "funcname": "process",
            "counter": 1000,
            "minwaittime": 0.1,
            "maxwaittime": 10.5,
            "minexectime": 1.0,
            "maxexectime": 60.0,
            "avgwaittime": 2.5,
            "avgexectime": 15.0
        }"#;

        let func: Function = serde_json::from_str(json).unwrap();
        assert_eq!(func.counter, 1000);
        assert_eq!(func.minwaittime, 0.1);
        assert_eq!(func.maxwaittime, 10.5);
        assert_eq!(func.avgexectime, 15.0);
    }

    #[test]
    fn test_processgraph_full() {
        let json = r#"{
            "processgraphid": "pg-123",
            "colonyname": "colony",
            "state": 1,
            "rootprocessids": ["root1"],
            "processids": ["root1", "child1", "child2"],
            "waitingids": [],
            "runningids": ["root1"],
            "successfulids": [],
            "failedids": []
        }"#;

        let pg: ProcessGraph = serde_json::from_str(json).unwrap();
        assert_eq!(pg.state, RUNNING);
        assert_eq!(pg.rootprocessids.len(), 1);
        assert_eq!(pg.processids.len(), 3);
    }
}
