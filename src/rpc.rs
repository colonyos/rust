use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::core::Attribute;
use crate::core::Blueprint;
use crate::core::BlueprintDefinition;
use crate::core::Colony;
use crate::core::Executor;
use crate::core::Failure;
use crate::core::Function;
use crate::core::FunctionSpec;
use crate::core::Log;
use crate::core::WorkflowSpec;
use crate::crypto;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::RwLock;

// Global server configuration
static SERVER_URL: RwLock<Option<String>> = RwLock::new(None);
const DEFAULT_SERVER_URL: &str = "http://localhost:50080/api";

/// Set the ColonyOS server URL for all API calls.
///
/// # Example
/// ```
/// colonyos::set_server_url("http://myserver:50080/api");
/// ```
pub fn set_server_url(url: &str) {
    let mut server_url = SERVER_URL.write().unwrap();
    *server_url = Some(url.to_string());
}

/// Get the current server URL.
pub fn get_server_url() -> String {
    let server_url = SERVER_URL.read().unwrap();
    server_url.clone().unwrap_or_else(|| DEFAULT_SERVER_URL.to_string())
}

// add colony

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddColonyRPCMsg {
    pub colony: Colony,
    pub msgtype: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveColonyRPCMsg {
    pub colonyname: String ,
    pub msgtype: String,
}


pub(super) fn compose_add_colony_rpcmsg(colony: &Colony, prvkey: &String) -> std::string::String {
    let payloadtype = "addcolonymsg";
    let add_colony_rpcmsg = AddColonyRPCMsg {
        colony: colony.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&add_colony_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

pub(super) fn compose_remove_colony_rpcmsg(colonyname: &String, prvkey: &String) -> std::string::String {
    let payloadtype = "removecolonymsg";
    let remove_colony_rpcmsg = RemoveColonyRPCMsg {
        colonyname: colonyname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&remove_colony_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// add executor

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddExecutorRPCMsg {
    pub executor: Executor,
    pub msgtype: String,
}

pub(super) fn compose_add_executor_rpcmsg(
    executor: &Executor,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "addexecutormsg";
    let add_executor_rpcmsg = AddExecutorRPCMsg {
        executor: executor.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&add_executor_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// approve executor

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ApproveExecutorRPCMsg {
    pub colonyname: String,
    pub executorname: String,
    pub msgtype: String,
}

pub(super) fn compose_approve_executor_rpcmsg(
    colonyname: &String,
    executorname: &String,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "approveexecutormsg";
    let approve_executor_rpcmsg = ApproveExecutorRPCMsg {
        colonyname: colonyname.to_owned(),
        executorname: executorname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&approve_executor_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// submit processspec

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubmitFunctionSpecRPCMsg {
    pub spec: FunctionSpec,
    pub msgtype: String,
}

pub(super) fn compose_submit_functionspec_rpcmsg(
    spec: &FunctionSpec,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "submitfuncspecmsg";
    let submit_processspec_rpcmsg = SubmitFunctionSpecRPCMsg {
        spec: spec.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&submit_processspec_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// assign process

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AssignProcessRPCMsg {
    pub colonyname: String,
    pub timeout: i32,
    pub msgtype: String,
}

pub(super) fn compose_assign_process_rpcmsg(
    colonyname: &String,
    timeout: i32,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "assignprocessmsg";
    let assign_process_rpcmsg = AssignProcessRPCMsg {
        colonyname: colonyname.to_owned(),
        timeout: timeout,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&assign_process_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// close process (as successful)

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CloseProcessRPCMsg {
    pub processid: String,
    pub msgtype: String,
}

pub(super) fn compose_close_process_rpcmsg(
    processid: &String,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "closesuccessfulmsg";
    let close_process_rpcmsg = CloseProcessRPCMsg {
        processid: processid.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&close_process_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// close process (as failed)

#[derive(Debug, Clone, Deserialize, Serialize)]
struct FailProcessRPCMsg {
    pub processid: String,
    pub msgtype: String,
}

pub(super) fn compose_fail_process_rpcmsg(
    processid: &String,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "closefailedmsg";
    let fail_process_rpcmsg = FailProcessRPCMsg {
        processid: processid.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&fail_process_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// add attribute

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddAttributeRPCMsg {
    pub attribute: Attribute,
    pub msgtype: String,
}

pub(super) fn compose_add_attr_rpcmsg(attr: &Attribute, prvkey: &String) -> std::string::String {
    let payloadtype = "addattributemsg";
    let add_attr_rpcmsg = AddAttributeRPCMsg {
        attribute: attr.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&add_attr_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// get process

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetProcessRPCMsg {
    pub processid: String,
    pub msgtype: String,
}

pub(super) fn compose_get_process_rpcmsg(
    processid: &String,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "getprocessmsg";
    let get_process_rpcmsg = GetProcessRPCMsg {
        processid: processid.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&get_process_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// get processes

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetProcessesRPCMsg {
    pub colonyname: String,
    pub count: i32,
    pub state: i32,
    pub msgtype: String,
}

pub(super) fn compose_get_processes_rpcmsg(
    colonyname: &String,
    count: i32,
    state: i32,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "getprocessesmsg";
    let get_processes_rpcmsg = GetProcessesRPCMsg {
        colonyname: colonyname.to_owned(),
        count: count,
        state: state,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&get_processes_rpcmsg).unwrap();
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg).unwrap()
}

// get colony

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetColonyRPCMsg {
    pub colonyname: String,
    pub msgtype: String,
}

pub(super) fn compose_get_colony_rpcmsg(colonyname: &str, prvkey: &str) -> String {
    let payloadtype = "getcolonymsg";
    let msg = GetColonyRPCMsg {
        colonyname: colonyname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get colonies

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetColoniesRPCMsg {
    pub msgtype: String,
}

pub(super) fn compose_get_colonies_rpcmsg(prvkey: &str) -> String {
    let payloadtype = "getcoloniesmsg";
    let msg = GetColoniesRPCMsg {
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get executor

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetExecutorRPCMsg {
    pub colonyname: String,
    pub executorname: String,
    pub msgtype: String,
}

pub(super) fn compose_get_executor_rpcmsg(colonyname: &str, executorname: &str, prvkey: &str) -> String {
    let payloadtype = "getexecutormsg";
    let msg = GetExecutorRPCMsg {
        colonyname: colonyname.to_owned(),
        executorname: executorname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get executors

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetExecutorsRPCMsg {
    pub colonyname: String,
    pub msgtype: String,
}

pub(super) fn compose_get_executors_rpcmsg(colonyname: &str, prvkey: &str) -> String {
    let payloadtype = "getexecutorsmsg";
    let msg = GetExecutorsRPCMsg {
        colonyname: colonyname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// reject executor

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RejectExecutorRPCMsg {
    pub colonyname: String,
    pub executorname: String,
    pub msgtype: String,
}

pub(super) fn compose_reject_executor_rpcmsg(colonyname: &str, executorname: &str, prvkey: &str) -> String {
    let payloadtype = "rejectexecutormsg";
    let msg = RejectExecutorRPCMsg {
        colonyname: colonyname.to_owned(),
        executorname: executorname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// remove executor

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveExecutorRPCMsg {
    pub colonyname: String,
    pub executorname: String,
    pub msgtype: String,
}

pub(super) fn compose_remove_executor_rpcmsg(colonyname: &str, executorname: &str, prvkey: &str) -> String {
    let payloadtype = "removeexecutormsg";
    let msg = RemoveExecutorRPCMsg {
        colonyname: colonyname.to_owned(),
        executorname: executorname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// remove process

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveProcessRPCMsg {
    pub processid: String,
    pub msgtype: String,
}

pub(super) fn compose_remove_process_rpcmsg(processid: &str, prvkey: &str) -> String {
    let payloadtype = "removeprocessmsg";
    let msg = RemoveProcessRPCMsg {
        processid: processid.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// remove all processes

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveAllProcessesRPCMsg {
    pub colonyname: String,
    pub state: i32,
    pub msgtype: String,
}

pub(super) fn compose_remove_all_processes_rpcmsg(colonyname: &str, state: i32, prvkey: &str) -> String {
    let payloadtype = "removeallprocessesmsg";
    let msg = RemoveAllProcessesRPCMsg {
        colonyname: colonyname.to_owned(),
        state,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// set output

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SetOutputRPCMsg {
    pub processid: String,
    pub out: Vec<String>,
    pub msgtype: String,
}

pub(super) fn compose_set_output_rpcmsg(processid: &str, output: Vec<String>, prvkey: &str) -> String {
    let payloadtype = "setoutputmsg";
    let msg = SetOutputRPCMsg {
        processid: processid.to_owned(),
        out: output,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// submit workflow

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubmitWorkflowRPCMsg {
    pub spec: WorkflowSpec,
    pub msgtype: String,
}

pub(super) fn compose_submit_workflow_rpcmsg(workflowspec: &WorkflowSpec, prvkey: &str) -> String {
    let payloadtype = "submitworkflowspecmsg";
    let msg = SubmitWorkflowRPCMsg {
        spec: workflowspec.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get process graph

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetProcessGraphRPCMsg {
    pub processgraphid: String,
    pub msgtype: String,
}

pub(super) fn compose_get_processgraph_rpcmsg(processgraphid: &str, prvkey: &str) -> String {
    let payloadtype = "getprocessgraphmsg";
    let msg = GetProcessGraphRPCMsg {
        processgraphid: processgraphid.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get process graphs

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetProcessGraphsRPCMsg {
    pub colonyname: String,
    pub count: i32,
    pub state: i32,
    pub msgtype: String,
}

pub(super) fn compose_get_processgraphs_rpcmsg(colonyname: &str, count: i32, state: i32, prvkey: &str) -> String {
    let payloadtype = "getprocessgraphsmsg";
    let msg = GetProcessGraphsRPCMsg {
        colonyname: colonyname.to_owned(),
        count,
        state,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// remove process graph

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveProcessGraphRPCMsg {
    pub processgraphid: String,
    pub msgtype: String,
}

pub(super) fn compose_remove_processgraph_rpcmsg(processgraphid: &str, prvkey: &str) -> String {
    let payloadtype = "removeprocessgraphmsg";
    let msg = RemoveProcessGraphRPCMsg {
        processgraphid: processgraphid.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// remove all process graphs

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveAllProcessGraphsRPCMsg {
    pub colonyname: String,
    pub state: i32,
    pub msgtype: String,
}

pub(super) fn compose_remove_all_processgraphs_rpcmsg(colonyname: &str, state: i32, prvkey: &str) -> String {
    let payloadtype = "removeallprocessgraphsmsg";
    let msg = RemoveAllProcessGraphsRPCMsg {
        colonyname: colonyname.to_owned(),
        state,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// add log

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddLogRPCMsg {
    pub processid: String,
    pub colonyname: String,
    pub executorname: String,
    pub message: String,
    pub msgtype: String,
}

pub(super) fn compose_add_log_rpcmsg(log: &Log, prvkey: &str) -> String {
    let payloadtype = "addlogmsg";
    let msg = AddLogRPCMsg {
        processid: log.processid.clone(),
        colonyname: log.colonyname.clone(),
        executorname: log.executorname.clone(),
        message: log.message.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get logs

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetLogsRPCMsg {
    pub colonyname: String,
    pub processid: String,
    pub executorname: String,
    pub count: i32,
    pub since: i64,
    pub msgtype: String,
}

pub(super) fn compose_get_logs_rpcmsg(
    colonyname: &str,
    processid: &str,
    executorname: &str,
    count: i32,
    since: i64,
    prvkey: &str,
) -> String {
    let payloadtype = "getlogsmsg";
    let msg = GetLogsRPCMsg {
        colonyname: colonyname.to_owned(),
        processid: processid.to_owned(),
        executorname: executorname.to_owned(),
        count,
        since,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// channel append

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ChannelAppendRPCMsg {
    pub msgtype: String,
    pub processid: String,
    pub name: String,
    pub sequence: i64,
    pub inreplyto: i64,
    pub payload: Vec<u8>,
}

pub(super) fn compose_channel_append_rpcmsg(
    processid: &str,
    channelname: &str,
    sequence: i64,
    data: &str,
    _data_type: &str,
    inreplyto: i64,
    prvkey: &str,
) -> String {
    let payloadtype = "channelappendmsg";
    let msg = ChannelAppendRPCMsg {
        msgtype: payloadtype.to_owned(),
        processid: processid.to_owned(),
        name: channelname.to_owned(),
        sequence,
        inreplyto,
        payload: data.as_bytes().to_vec(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// channel read

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ChannelReadRPCMsg {
    pub processid: String,
    pub name: String,
    pub afterseq: i64,
    pub limit: i32,
    pub msgtype: String,
}

pub(super) fn compose_channel_read_rpcmsg(
    processid: &str,
    channelname: &str,
    afterseq: i64,
    limit: i32,
    prvkey: &str,
) -> String {
    let payloadtype = "channelreadmsg";
    let msg = ChannelReadRPCMsg {
        processid: processid.to_owned(),
        name: channelname.to_owned(),
        afterseq,
        limit,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get statistics

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetStatisticsRPCMsg {
    pub colonyname: String,
    pub msgtype: String,
}

pub(super) fn compose_get_statistics_rpcmsg(colonyname: &str, prvkey: &str) -> String {
    let payloadtype = "getcolonystatsmsg";
    let msg = GetStatisticsRPCMsg {
        colonyname: colonyname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// add function

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddFunctionRPCMsg {
    pub function: Function,
    pub msgtype: String,
}

pub(super) fn compose_add_function_rpcmsg(function: &Function, prvkey: &str) -> String {
    let payloadtype = "addfunctionmsg";
    let msg = AddFunctionRPCMsg {
        function: function.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get functions

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetFunctionsRPCMsg {
    pub colonyname: String,
    pub msgtype: String,
}

pub(super) fn compose_get_functions_rpcmsg(colonyname: &str, prvkey: &str) -> String {
    let payloadtype = "getfunctionsmsg";
    let msg = GetFunctionsRPCMsg {
        colonyname: colonyname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get functions by executor

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetFunctionsByExecutorRPCMsg {
    pub colonyname: String,
    pub executorname: String,
    pub msgtype: String,
}

pub(super) fn compose_get_functions_by_executor_rpcmsg(colonyname: &str, executorname: &str, prvkey: &str) -> String {
    let payloadtype = "getfunctionsbyexecutormsg";
    let msg = GetFunctionsByExecutorRPCMsg {
        colonyname: colonyname.to_owned(),
        executorname: executorname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// remove function

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveFunctionRPCMsg {
    pub functionid: String,
    pub msgtype: String,
}

pub(super) fn compose_remove_function_rpcmsg(functionid: &str, prvkey: &str) -> String {
    let payloadtype = "removefunctionmsg";
    let msg = RemoveFunctionRPCMsg {
        functionid: functionid.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// add blueprint definition

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddBlueprintDefinitionRPCMsg {
    pub blueprintdefinition: BlueprintDefinition,
    pub msgtype: String,
}

pub(super) fn compose_add_blueprint_definition_rpcmsg(definition: &BlueprintDefinition, prvkey: &str) -> String {
    let payloadtype = "addblueprintdefinitionmsg";
    let msg = AddBlueprintDefinitionRPCMsg {
        blueprintdefinition: definition.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get blueprint definition

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetBlueprintDefinitionRPCMsg {
    pub colonyname: String,
    pub name: String,
    pub msgtype: String,
}

pub(super) fn compose_get_blueprint_definition_rpcmsg(colonyname: &str, name: &str, prvkey: &str) -> String {
    let payloadtype = "getblueprintdefinitionmsg";
    let msg = GetBlueprintDefinitionRPCMsg {
        colonyname: colonyname.to_owned(),
        name: name.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get blueprint definitions

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetBlueprintDefinitionsRPCMsg {
    pub colonyname: String,
    pub msgtype: String,
}

pub(super) fn compose_get_blueprint_definitions_rpcmsg(colonyname: &str, prvkey: &str) -> String {
    let payloadtype = "getblueprintdefinitionsmsg";
    let msg = GetBlueprintDefinitionsRPCMsg {
        colonyname: colonyname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// remove blueprint definition

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveBlueprintDefinitionRPCMsg {
    pub colonyname: String,
    pub name: String,
    pub msgtype: String,
}

pub(super) fn compose_remove_blueprint_definition_rpcmsg(colonyname: &str, name: &str, prvkey: &str) -> String {
    let payloadtype = "removeblueprintdefinitionmsg";
    let msg = RemoveBlueprintDefinitionRPCMsg {
        colonyname: colonyname.to_owned(),
        name: name.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// add blueprint

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddBlueprintRPCMsg {
    pub blueprint: Blueprint,
    pub msgtype: String,
}

pub(super) fn compose_add_blueprint_rpcmsg(blueprint: &Blueprint, prvkey: &str) -> String {
    let payloadtype = "addblueprintmsg";
    let msg = AddBlueprintRPCMsg {
        blueprint: blueprint.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get blueprint

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetBlueprintRPCMsg {
    pub colonyname: String,
    pub name: String,
    pub msgtype: String,
}

pub(super) fn compose_get_blueprint_rpcmsg(colonyname: &str, name: &str, prvkey: &str) -> String {
    let payloadtype = "getblueprintmsg";
    let msg = GetBlueprintRPCMsg {
        colonyname: colonyname.to_owned(),
        name: name.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// get blueprints

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetBlueprintsRPCMsg {
    pub colonyname: String,
    pub kind: String,
    pub location: String,
    pub msgtype: String,
}

pub(super) fn compose_get_blueprints_rpcmsg(colonyname: &str, kind: &str, location: &str, prvkey: &str) -> String {
    let payloadtype = "getblueprintsmsg";
    let msg = GetBlueprintsRPCMsg {
        colonyname: colonyname.to_owned(),
        kind: kind.to_owned(),
        location: location.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// update blueprint

#[derive(Debug, Clone, Deserialize, Serialize)]
struct UpdateBlueprintRPCMsg {
    pub blueprint: Blueprint,
    pub forcegeneration: bool,
    pub msgtype: String,
}

pub(super) fn compose_update_blueprint_rpcmsg(blueprint: &Blueprint, force_generation: bool, prvkey: &str) -> String {
    let payloadtype = "updateblueprintmsg";
    let msg = UpdateBlueprintRPCMsg {
        blueprint: blueprint.clone(),
        forcegeneration: force_generation,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// remove blueprint

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveBlueprintRPCMsg {
    pub colonyname: String,
    pub name: String,
    pub msgtype: String,
}

pub(super) fn compose_remove_blueprint_rpcmsg(colonyname: &str, name: &str, prvkey: &str) -> String {
    let payloadtype = "removeblueprintmsg";
    let msg = RemoveBlueprintRPCMsg {
        colonyname: colonyname.to_owned(),
        name: name.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// update blueprint status

#[derive(Debug, Clone, Deserialize, Serialize)]
struct UpdateBlueprintStatusRPCMsg {
    pub colonyname: String,
    pub name: String,
    pub status: HashMap<String, Value>,
    pub msgtype: String,
}

pub(super) fn compose_update_blueprint_status_rpcmsg(
    colonyname: &str,
    name: &str,
    status: HashMap<String, Value>,
    prvkey: &str,
) -> String {
    let payloadtype = "updateblueprintstatusmsg";
    let msg = UpdateBlueprintStatusRPCMsg {
        colonyname: colonyname.to_owned(),
        name: name.to_owned(),
        status,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// reconcile blueprint

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ReconcileBlueprintRPCMsg {
    pub colonyname: String,
    pub name: String,
    pub force: bool,
    pub msgtype: String,
}

pub(super) fn compose_reconcile_blueprint_rpcmsg(colonyname: &str, name: &str, force: bool, prvkey: &str) -> String {
    let payloadtype = "reconcileblueprintmsg";
    let msg = ReconcileBlueprintRPCMsg {
        colonyname: colonyname.to_owned(),
        name: name.to_owned(),
        force,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// subscribe process

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubscribeProcessRPCMsg {
    pub processid: String,
    pub executortype: String,
    pub state: i32,
    pub timeout: i32,
    pub colonyname: String,
    pub msgtype: String,
}

pub(super) fn compose_subscribe_process_rpcmsg(
    processid: &str,
    executortype: &str,
    state: i32,
    timeout: i32,
    colonyname: &str,
    prvkey: &str,
) -> String {
    let payloadtype = "subscribeprocessmsg";
    let msg = SubscribeProcessRPCMsg {
        processid: processid.to_owned(),
        executortype: executortype.to_owned(),
        state,
        timeout,
        colonyname: colonyname.to_owned(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// subscribe channel

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubscribeChannelRPCMsg {
    pub processid: String,
    pub name: String,
    pub afterseq: i64,
    pub timeout: i32,
    pub msgtype: String,
}

pub(super) fn compose_subscribe_channel_rpcmsg(
    processid: &str,
    channelname: &str,
    afterseq: i64,
    timeout: i32,
    prvkey: &str,
) -> String {
    let payloadtype = "subscribechannelmsg";
    let msg = SubscribeChannelRPCMsg {
        processid: processid.to_owned(),
        name: channelname.to_owned(),
        afterseq,
        timeout,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// RPC

impl Error for RPCError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct RPCError {
    details: String,
    connection_error: bool,
}

impl RPCError {
    fn new(msg: &str, connection_error: bool) -> RPCError {
        RPCError {
            details: msg.to_string(),
            connection_error: connection_error,
        }
    }
    pub fn conn_err(&self) -> bool {
        self.connection_error
    }
}

impl fmt::Display for RPCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct RPCMsg {
    pub signature: String,
     pub payloadtype: String,
    pub payload: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RPCReplyMsg {
    pub payloadtype: String,
    pub payload: String,
    pub error: bool,
}

fn compose_rpcmsg(payloadtype: String, payload: String, prvkey: String) -> RPCMsg {
    let payload_base64 = BASE64.encode(payload.as_bytes());
    let signature = crypto::gen_signature(&payload_base64, &prvkey);
    RPCMsg {
        payload: payload_base64,
        payloadtype: payloadtype,
        signature: signature,
    }
}

pub(super) async fn send_rpcmsg(msg: String) -> Result<String, RPCError> {
    let server_url = get_server_url();
    let client = reqwest::Client::new();
    let res = client
        .post(&server_url)
        .body(msg)
        .send()
        .await;

    let res = match res {
        Ok(res) => res,
        Err(err) => return Err(RPCError::new(&err.to_string(), true)),
    };

    let status = res.status();

    let body = res.text().await;
    let body = match body {
        Ok(body) => body,
        Err(err) => return Err(RPCError::new(&err.to_string(), false)),
    };

    let rpc_reply: RPCReplyMsg = serde_json::from_str(body.as_str())
        .map_err(|e| RPCError::new(&format!("Failed to parse response: {} - body: {}", e, body), false))?;
    let buf = BASE64.decode(rpc_reply.payload.as_str())
        .map_err(|e| RPCError::new(&format!("Failed to decode payload: {}", e), false))?;
    let s = String::from_utf8(buf)
        .map_err(|e| RPCError::new(&format!("Invalid UTF-8 in payload: {}", e), false))?;

    if status != 200 {
        let failure: Failure = serde_json::from_str(s.as_str())
            .map_err(|e| RPCError::new(&format!("Failed to parse error: {} - body: {}", e, s), false))?;
        return Err(RPCError::new(failure.message.as_str(), false));
    }

    Ok(s)
}

/// Get the WebSocket URL from the current server URL
fn get_ws_url() -> String {
    let http_url = get_server_url();
    // Replace http:// with ws:// and https:// with wss://
    // Also replace /api with /pubsub
    let ws_url = if http_url.starts_with("https://") {
        http_url.replace("https://", "wss://")
    } else {
        http_url.replace("http://", "ws://")
    };
    ws_url.replace("/api", "/pubsub")
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) async fn send_ws_subscribe_process(msg: String) -> Result<(), RPCError> {
    use tokio_tungstenite::connect_async;
    use futures_util::{SinkExt, StreamExt};

    let ws_url = get_ws_url();

    let (ws_stream, _) = connect_async(&ws_url)
        .await
        .map_err(|e| RPCError::new(&format!("WebSocket connection failed: {}", e), true))?;

    let (mut write, mut read) = ws_stream.split();

    // Send the subscription message
    write
        .send(tokio_tungstenite::tungstenite::Message::Text(msg))
        .await
        .map_err(|e| RPCError::new(&format!("WebSocket send failed: {}", e), true))?;

    // Wait for a response (process state change notification)
    if let Some(msg) = read.next().await {
        match msg {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                let rpc_reply: RPCReplyMsg = serde_json::from_str(&text)
                    .map_err(|e| RPCError::new(&format!("Failed to parse WebSocket response: {}", e), false))?;

                if rpc_reply.error {
                    let buf = BASE64.decode(rpc_reply.payload.as_str()).unwrap();
                    let s = String::from_utf8(buf).expect("valid byte array");
                    let failure: Failure = serde_json::from_str(&s).unwrap();
                    return Err(RPCError::new(&failure.message, false));
                }
            }
            Ok(_) => {}
            Err(e) => {
                return Err(RPCError::new(&format!("WebSocket error: {}", e), true));
            }
        }
    }

    // Close the connection
    write.close().await.ok();

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) async fn send_ws_subscribe_channel<F>(
    msg: String,
    timeout_secs: i32,
    mut callback: F,
) -> Result<Vec<crate::core::ChannelEntry>, RPCError>
where
    F: FnMut(Vec<crate::core::ChannelEntry>) -> bool,
{
    use tokio_tungstenite::connect_async;
    use futures_util::{SinkExt, StreamExt};
    use tokio::time::{timeout, Duration};

    let ws_url = get_ws_url();

    // Add extra time for connection overhead
    let client_timeout = Duration::from_secs((timeout_secs as u64) + 5);

    let connect_result = timeout(Duration::from_secs(10), connect_async(&ws_url)).await;
    let (ws_stream, _) = match connect_result {
        Ok(Ok(stream)) => stream,
        Ok(Err(e)) => return Err(RPCError::new(&format!("WebSocket connection failed: {}", e), true)),
        Err(_) => return Err(RPCError::new("WebSocket connection timed out", true)),
    };

    let (mut write, mut read) = ws_stream.split();

    // Send the subscription message
    write
        .send(tokio_tungstenite::tungstenite::Message::Text(msg))
        .await
        .map_err(|e| RPCError::new(&format!("WebSocket send failed: {}", e), true))?;

    let mut all_entries = Vec::new();

    // Read messages until timeout or stream ends
    loop {
        match timeout(client_timeout, read.next()).await {
            Ok(Some(msg)) => {
                match msg {
                    Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                        let rpc_reply: RPCReplyMsg = serde_json::from_str(&text)
                            .map_err(|e| RPCError::new(&format!("Failed to parse WebSocket response: {}", e), false))?;

                        if rpc_reply.error {
                            let buf = BASE64.decode(rpc_reply.payload.as_str()).unwrap();
                            let s = String::from_utf8(buf).expect("valid byte array");
                            let failure: Failure = serde_json::from_str(&s).unwrap();
                            return Err(RPCError::new(&failure.message, false));
                        }

                        let buf = BASE64.decode(rpc_reply.payload.as_str()).unwrap();
                        let s = String::from_utf8(buf).expect("valid byte array");
                        let entries: Vec<crate::core::ChannelEntry> = serde_json::from_str(&s).unwrap_or_default();

                        if entries.is_empty() {
                            // Empty response indicates server-side timeout
                            break;
                        }

                        all_entries.extend(entries.clone());

                        // Call callback and check if we should continue
                        if !callback(entries) {
                            break;
                        }
                    }
                    Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => break,
                    Ok(_) => {}
                    Err(e) => {
                        return Err(RPCError::new(&format!("WebSocket error: {}", e), true));
                    }
                }
            }
            Ok(None) => break, // Stream ended
            Err(_) => break,   // Client-side timeout
        }
    }

    // Close the connection
    write.close().await.ok();

    Ok(all_entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Colony, Executor, Function, FunctionSpec, Conditions, Attribute, Log, WorkflowSpec};

    const TEST_PRVKEY: &str = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";

    #[test]
    fn test_rpc_error_creation() {
        let err = RPCError::new("test error", false);
        assert_eq!(err.details, "test error");
        assert!(!err.conn_err());

        let conn_err = RPCError::new("connection failed", true);
        assert!(conn_err.conn_err());
    }

    #[test]
    fn test_rpc_error_display() {
        let err = RPCError::new("test error message", false);
        assert_eq!(format!("{}", err), "test error message");
    }

    #[test]
    fn test_rpc_error_description() {
        let err = RPCError::new("error details", false);
        use std::error::Error;
        assert_eq!(err.description(), "error details");
    }

    #[test]
    fn test_compose_rpcmsg_structure() {
        let rpcmsg = compose_rpcmsg(
            "testmsg".to_string(),
            r#"{"test":"data"}"#.to_string(),
            TEST_PRVKEY.to_string(),
        );
        assert_eq!(rpcmsg.payloadtype, "testmsg");
        assert!(!rpcmsg.payload.is_empty());
        assert!(!rpcmsg.signature.is_empty());

        // Verify payload is base64 encoded
        let decoded = BASE64.decode(&rpcmsg.payload).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        assert_eq!(decoded_str, r#"{"test":"data"}"#);
    }

    #[test]
    fn test_compose_add_colony_rpcmsg() {
        let colony = Colony::new("colony-id", "test-colony");
        let msg = compose_add_colony_rpcmsg(&colony, &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "addcolonymsg");
        assert!(!parsed["payload"].as_str().unwrap().is_empty());
        assert!(!parsed["signature"].as_str().unwrap().is_empty());
    }

    #[test]
    fn test_compose_remove_colony_rpcmsg() {
        let msg = compose_remove_colony_rpcmsg(&"test-colony".to_string(), &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "removecolonymsg");
    }

    #[test]
    fn test_compose_add_executor_rpcmsg() {
        let executor = Executor::new("exec-name", "exec-id", "cli", "test-colony");
        let msg = compose_add_executor_rpcmsg(&executor, &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "addexecutormsg");
    }

    #[test]
    fn test_compose_approve_executor_rpcmsg() {
        let msg = compose_approve_executor_rpcmsg(
            &"test-colony".to_string(),
            &"test-executor".to_string(),
            &TEST_PRVKEY.to_string(),
        );
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "approveexecutormsg");
    }

    #[test]
    fn test_approve_executor_with_colony_key() {
        // Use the colony key from docker-compose.env
        let colony_key = "ba949fa134981372d6da62b6a56f336ab4d843b22c02a4257dcf7d0d73097514";
        let colony_id = crypto::gen_id(colony_key);

        // Create the approve message
        let msg = compose_approve_executor_rpcmsg(
            &"dev".to_string(),
            &"wasm-executor".to_string(),
            &colony_key.to_string(),
        );

        // Parse and extract the payload
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        let payload_b64 = parsed["payload"].as_str().unwrap();
        let signature = parsed["signature"].as_str().unwrap();

        // Decode payload and verify structure
        let payload_bytes = BASE64.decode(payload_b64.as_bytes()).unwrap();
        let payload_str = String::from_utf8(payload_bytes).unwrap();

        println!("Colony ID: {}", colony_id);
        println!("Payload JSON: {}", payload_str);
        println!("Payload Base64: {}", payload_b64);
        println!("Signature: {}", signature);

        // Verify signature recovery
        let recovered_id = crypto::recid(payload_b64, signature);
        println!("Recovered ID: {}", recovered_id);

        assert_eq!(colony_id, recovered_id, "Recovered ID should match colony ID");
    }

    #[test]
    fn test_compose_reject_executor_rpcmsg() {
        let msg = compose_reject_executor_rpcmsg("test-colony", "test-executor", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "rejectexecutormsg");
    }

    #[test]
    fn test_compose_remove_executor_rpcmsg() {
        let msg = compose_remove_executor_rpcmsg("test-colony", "test-executor", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "removeexecutormsg");
    }

    #[test]
    fn test_compose_get_executor_rpcmsg() {
        let msg = compose_get_executor_rpcmsg("test-colony", "test-executor", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getexecutormsg");
    }

    #[test]
    fn test_compose_get_executors_rpcmsg() {
        let msg = compose_get_executors_rpcmsg("test-colony", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getexecutorsmsg");
    }

    #[test]
    fn test_compose_submit_functionspec_rpcmsg() {
        let spec = FunctionSpec::new("test-func", "cli", "test-colony");
        let msg = compose_submit_functionspec_rpcmsg(&spec, &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "submitfuncspecmsg");
    }

    #[test]
    fn test_compose_assign_process_rpcmsg() {
        let msg = compose_assign_process_rpcmsg(&"test-colony".to_string(), 30, &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "assignprocessmsg");
    }

    #[test]
    fn test_compose_close_process_rpcmsg() {
        let msg = compose_close_process_rpcmsg(&"process-123".to_string(), &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "closesuccessfulmsg");
    }

    #[test]
    fn test_compose_fail_process_rpcmsg() {
        let msg = compose_fail_process_rpcmsg(&"process-123".to_string(), &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "closefailedmsg");
    }

    #[test]
    fn test_compose_get_process_rpcmsg() {
        let msg = compose_get_process_rpcmsg(&"process-123".to_string(), &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getprocessmsg");
    }

    #[test]
    fn test_compose_get_processes_rpcmsg() {
        let msg = compose_get_processes_rpcmsg(&"test-colony".to_string(), 100, 0, &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getprocessesmsg");
    }

    #[test]
    fn test_compose_remove_process_rpcmsg() {
        let msg = compose_remove_process_rpcmsg("process-123", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "removeprocessmsg");
    }

    #[test]
    fn test_compose_remove_all_processes_rpcmsg() {
        let msg = compose_remove_all_processes_rpcmsg("test-colony", 0, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "removeallprocessesmsg");
    }

    #[test]
    fn test_compose_get_colony_rpcmsg() {
        let msg = compose_get_colony_rpcmsg("test-colony", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getcolonymsg");
    }

    #[test]
    fn test_compose_get_colonies_rpcmsg() {
        let msg = compose_get_colonies_rpcmsg(TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getcoloniesmsg");
    }

    #[test]
    fn test_compose_add_attr_rpcmsg() {
        let attr = Attribute::new("test-colony", "process-123", "key", "value");
        let msg = compose_add_attr_rpcmsg(&attr, &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "addattributemsg");
    }

    #[test]
    fn test_compose_set_output_rpcmsg() {
        let msg = compose_set_output_rpcmsg("process-123", vec!["out1".to_string()], TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "setoutputmsg");
    }

    #[test]
    fn test_compose_submit_workflow_rpcmsg() {
        let spec = FunctionSpec::new("test-func", "cli", "test-colony");
        let workflow = WorkflowSpec {
            colonyname: "test-colony".to_string(),
            functionspecs: vec![spec],
        };
        let msg = compose_submit_workflow_rpcmsg(&workflow, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "submitworkflowspecmsg");
    }

    #[test]
    fn test_compose_get_processgraph_rpcmsg() {
        let msg = compose_get_processgraph_rpcmsg("graph-123", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getprocessgraphmsg");
    }

    #[test]
    fn test_compose_get_processgraphs_rpcmsg() {
        let msg = compose_get_processgraphs_rpcmsg("test-colony", 100, 0, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getprocessgraphsmsg");
    }

    #[test]
    fn test_compose_remove_processgraph_rpcmsg() {
        let msg = compose_remove_processgraph_rpcmsg("graph-123", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "removeprocessgraphmsg");
    }

    #[test]
    fn test_compose_remove_all_processgraphs_rpcmsg() {
        let msg = compose_remove_all_processgraphs_rpcmsg("test-colony", 0, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "removeallprocessgraphsmsg");
    }

    #[test]
    fn test_compose_add_log_rpcmsg() {
        let log = Log {
            processid: "process-123".to_string(),
            colonyname: "test-colony".to_string(),
            executorname: "test-executor".to_string(),
            message: "test message".to_string(),
            timestamp: 0,
        };
        let msg = compose_add_log_rpcmsg(&log, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "addlogmsg");
    }

    #[test]
    fn test_compose_get_logs_rpcmsg() {
        let msg = compose_get_logs_rpcmsg("test-colony", "process-123", "executor", 100, 0, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getlogsmsg");
    }

    #[test]
    fn test_compose_channel_append_rpcmsg() {
        let msg = compose_channel_append_rpcmsg("process-123", "channel1", 1, "hello", "", 0, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "channelappendmsg");
    }

    #[test]
    fn test_compose_channel_read_rpcmsg() {
        let msg = compose_channel_read_rpcmsg("process-123", "channel1", 0, 100, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "channelreadmsg");
    }

    #[test]
    fn test_compose_get_statistics_rpcmsg() {
        let msg = compose_get_statistics_rpcmsg("test-colony", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getcolonystatsmsg");
    }

    #[test]
    fn test_compose_add_function_rpcmsg() {
        let func = Function {
            functionid: "".to_string(),
            funcname: "test-func".to_string(),
            executorname: "test-executor".to_string(),
            executortype: "cli".to_string(),
            colonyname: "test-colony".to_string(),
            counter: 0,
            minwaittime: 0.0,
            maxwaittime: 0.0,
            minexectime: 0.0,
            maxexectime: 0.0,
            avgwaittime: 0.0,
            avgexectime: 0.0,
        };
        let msg = compose_add_function_rpcmsg(&func, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "addfunctionmsg");
    }

    #[test]
    fn test_compose_get_functions_rpcmsg() {
        let msg = compose_get_functions_rpcmsg("test-colony", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getfunctionsmsg");
    }

    #[test]
    fn test_compose_get_functions_by_executor_rpcmsg() {
        let msg = compose_get_functions_by_executor_rpcmsg("test-colony", "test-executor", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getfunctionsbyexecutormsg");
    }

    #[test]
    fn test_compose_remove_function_rpcmsg() {
        let msg = compose_remove_function_rpcmsg("func-123", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "removefunctionmsg");
    }

    #[test]
    fn test_compose_blueprint_definition_rpcmsgs() {
        let def = BlueprintDefinition::default();

        let msg = compose_add_blueprint_definition_rpcmsg(&def, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "addblueprintdefinitionmsg");

        let msg = compose_get_blueprint_definition_rpcmsg("colony", "name", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getblueprintdefinitionmsg");

        let msg = compose_get_blueprint_definitions_rpcmsg("colony", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getblueprintdefinitionsmsg");

        let msg = compose_remove_blueprint_definition_rpcmsg("colony", "name", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "removeblueprintdefinitionmsg");
    }

    #[test]
    fn test_compose_blueprint_rpcmsgs() {
        let bp = Blueprint::default();

        let msg = compose_add_blueprint_rpcmsg(&bp, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "addblueprintmsg");

        let msg = compose_get_blueprint_rpcmsg("colony", "name", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getblueprintmsg");

        let msg = compose_get_blueprints_rpcmsg("colony", "kind", "location", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "getblueprintsmsg");

        let msg = compose_update_blueprint_rpcmsg(&bp, true, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "updateblueprintmsg");

        let msg = compose_remove_blueprint_rpcmsg("colony", "name", TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "removeblueprintmsg");
    }

    #[test]
    fn test_compose_update_blueprint_status_rpcmsg() {
        let mut status = HashMap::new();
        status.insert("key".to_string(), serde_json::json!("value"));
        let msg = compose_update_blueprint_status_rpcmsg("colony", "name", status, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "updateblueprintstatusmsg");
    }

    #[test]
    fn test_compose_reconcile_blueprint_rpcmsg() {
        let msg = compose_reconcile_blueprint_rpcmsg("colony", "name", true, TEST_PRVKEY);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "reconcileblueprintmsg");
    }

    #[test]
    fn test_rpcmsg_payload_decoding() {
        let colony = Colony::new("id", "name");
        let msg = compose_add_colony_rpcmsg(&colony, &TEST_PRVKEY.to_string());
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();

        // Decode payload and verify structure
        let payload_b64 = parsed["payload"].as_str().unwrap();
        let payload_bytes = BASE64.decode(payload_b64).unwrap();
        let payload_str = String::from_utf8(payload_bytes).unwrap();
        let payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap();

        assert_eq!(payload_json["msgtype"], "addcolonymsg");
        assert_eq!(payload_json["colony"]["colonyid"], "id");
        assert_eq!(payload_json["colony"]["name"], "name");
    }

    #[test]
    fn test_compose_subscribe_process_rpcmsg() {
        let msg = compose_subscribe_process_rpcmsg(
            "process-123",
            "cli",
            2, // state: SUCCESS
            30,
            "test-colony",
            TEST_PRVKEY,
        );
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "subscribeprocessmsg");

        // Decode and verify payload contents
        let payload_b64 = parsed["payload"].as_str().unwrap();
        let payload_bytes = BASE64.decode(payload_b64).unwrap();
        let payload_str = String::from_utf8(payload_bytes).unwrap();
        let payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap();

        assert_eq!(payload_json["processid"], "process-123");
        assert_eq!(payload_json["executortype"], "cli");
        assert_eq!(payload_json["state"], 2);
        assert_eq!(payload_json["timeout"], 30);
        assert_eq!(payload_json["colonyname"], "test-colony");
        assert_eq!(payload_json["msgtype"], "subscribeprocessmsg");
    }

    #[test]
    fn test_compose_subscribe_channel_rpcmsg() {
        let msg = compose_subscribe_channel_rpcmsg(
            "process-123",
            "my-channel",
            10, // afterseq
            30, // timeout
            TEST_PRVKEY,
        );
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["payloadtype"], "subscribechannelmsg");

        // Decode and verify payload contents
        let payload_b64 = parsed["payload"].as_str().unwrap();
        let payload_bytes = BASE64.decode(payload_b64).unwrap();
        let payload_str = String::from_utf8(payload_bytes).unwrap();
        let payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap();

        assert_eq!(payload_json["processid"], "process-123");
        assert_eq!(payload_json["name"], "my-channel");
        assert_eq!(payload_json["afterseq"], 10);
        assert_eq!(payload_json["timeout"], 30);
        assert_eq!(payload_json["msgtype"], "subscribechannelmsg");
    }

    #[test]
    fn test_get_ws_url_http() {
        set_server_url("http://localhost:50080/api");
        let ws_url = get_ws_url();
        assert_eq!(ws_url, "ws://localhost:50080/pubsub");
    }

    #[test]
    fn test_get_ws_url_https() {
        set_server_url("https://secure.example.com:443/api");
        let ws_url = get_ws_url();
        assert_eq!(ws_url, "wss://secure.example.com:443/pubsub");

        // Reset to default
        set_server_url("http://localhost:50080/api");
    }
}
