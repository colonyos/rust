extern crate base64;
use crate::core::Attribute;
use crate::core::Colony;
use crate::core::Executor;
use crate::core::Failure;
use crate::core::FunctionSpec;
use crate::crypto;
use base64::{decode, encode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

// add colony

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddColonyRPCMsg {
    pub colony: Colony,
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
    pub executorid: String,
    pub msgtype: String,
}

pub(super) fn compose_approve_executor_rpcmsg(
    executorid: &String,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "approveexecutormsg";
    let approve_executor_rpcmsg = ApproveExecutorRPCMsg {
        executorid: executorid.to_owned(),
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
    pub colonyid: String,
    pub latest: bool,
    pub timeout: i32,
    pub msgtype: String,
}

pub(super) fn compose_assign_process_rpcmsg(
    colonyid: &String,
    latest: bool,
    timeout: i32,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "assignprocessmsg";
    let assign_process_rpcmsg = AssignProcessRPCMsg {
        colonyid: colonyid.to_owned(),
        latest: latest,
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
    pub colonyid: String,
    pub count: i32,
    pub state: i32,
    pub msgtype: String,
}

pub(super) fn compose_get_processes_rpcmsg(
    colonyid: &String,
    count: i32,
    state: i32,
    prvkey: &String,
) -> std::string::String {
    let payloadtype = "getprocessesmsg";
    let get_processes_rpcmsg = GetProcessesRPCMsg {
        colonyid: colonyid.to_owned(),
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
    let payload_base64 = encode(payload.as_bytes());
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
    let buf = decode(rpc_reply.payload.as_str()).unwrap();
    let s = String::from_utf8(buf).expect("valid byte array");

    if status != 200 {
        let failure: Failure = serde_json::from_str(s.as_str()).unwrap();
        return Err(RPCError::new(failure.message.as_str(), false));
    }

    Ok(s)
}
