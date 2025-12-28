//! ColonyOS Rust SDK
//!
//! This library provides a Rust client for interacting with ColonyOS servers.
//!
//! # Example
//!
//! ```rust,no_run
//! use colonies::{core, submit, assign, close};
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

// ============== Channel Methods ==============

pub async fn channel_append(
    processid: &str,
    channelname: &str,
    data: &str,
    data_type: &str,
    inreplyto: i64,
    prvkey: &str,
) -> Result<core::ChannelEntry, rpc::RPCError> {
    let rpcmsg = rpc::compose_channel_append_rpcmsg(processid, channelname, data, data_type, inreplyto, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let entry: core::ChannelEntry = serde_json::from_str(reply_json.as_str()).unwrap();
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
    use crate::core::*;

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
        let json = r#"{
            "sequence": 42,
            "data": "hello world",
            "type": "data",
            "inreplyto": 0
        }"#;

        let entry: ChannelEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.sequence, 42);
        assert_eq!(entry.data, "hello world");
        assert_eq!(entry.msgtype, "data");
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
}
