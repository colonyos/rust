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
    pub availablecpu: String,
    pub availablemem: String,
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
        availablecpu: "".to_owned(),
        availablemem: "".to_owned(),
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
    pub output: Vec<String>,
    pub msgtype: String,
}

pub(super) fn compose_set_output_rpcmsg(processid: &str, output: Vec<String>, prvkey: &str) -> String {
    let payloadtype = "setoutputmsg";
    let msg = SetOutputRPCMsg {
        processid: processid.to_owned(),
        output,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// submit workflow

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubmitWorkflowRPCMsg {
    pub workflowspec: WorkflowSpec,
    pub msgtype: String,
}

pub(super) fn compose_submit_workflow_rpcmsg(workflowspec: &WorkflowSpec, prvkey: &str) -> String {
    let payloadtype = "submitworkflowspecmsg";
    let msg = SubmitWorkflowRPCMsg {
        workflowspec: workflowspec.clone(),
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
    pub processid: String,
    pub channelname: String,
    pub data: String,
    #[serde(rename = "type")]
    pub msgtype_field: String,
    pub inreplyto: i64,
    pub msgtype: String,
}

pub(super) fn compose_channel_append_rpcmsg(
    processid: &str,
    channelname: &str,
    data: &str,
    data_type: &str,
    inreplyto: i64,
    prvkey: &str,
) -> String {
    let payloadtype = "channelappendmsg";
    let msg = ChannelAppendRPCMsg {
        processid: processid.to_owned(),
        channelname: channelname.to_owned(),
        data: data.to_owned(),
        msgtype_field: data_type.to_owned(),
        inreplyto,
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&msg).unwrap();
    let rpcmsg = compose_rpcmsg(payloadtype.to_owned(), payload, prvkey.to_owned());
    serde_json::to_string(&rpcmsg).unwrap()
}

// channel read

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ChannelReadRPCMsg {
    pub processid: String,
    pub channelname: String,
    pub start: i64,
    pub count: i32,
    pub msgtype: String,
}

pub(super) fn compose_channel_read_rpcmsg(
    processid: &str,
    channelname: &str,
    start: i64,
    count: i32,
    prvkey: &str,
) -> String {
    let payloadtype = "channelreadmsg";
    let msg = ChannelReadRPCMsg {
        processid: processid.to_owned(),
        channelname: channelname.to_owned(),
        start,
        count,
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
    let payloadtype = "getstatisticsmsg";
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
    let client = reqwest::Client::new();
    let res = client
        //.post("https://colonies.colonyos.io:443/api")
        .post("http://localhost:50080/api")
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

    let rpc_reply: RPCReplyMsg = serde_json::from_str(body.as_str()).unwrap();
    let buf = BASE64.decode(rpc_reply.payload.as_str()).unwrap();
    let s = String::from_utf8(buf).expect("valid byte array");

    if status != 200 {
        let failure: Failure = serde_json::from_str(s.as_str()).unwrap();
        return Err(RPCError::new(failure.message.as_str(), false));
    }

    Ok(s)
}
