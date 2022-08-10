extern crate base64;
use crate::colonieslib::core;
use crate::cryptolib::crypto;
use base64::{decode, encode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RPCError {
    details: String,
}

impl RPCError {
    fn new(msg: &str) -> RPCError {
        RPCError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for RPCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for RPCError {
    fn description(&self) -> &str {
        &self.details
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

#[derive(Debug, Clone, Deserialize, Serialize)]
struct AddColonyRPCMsg {
    pub colony: core::Colony,
    pub msgtype: String,
}

pub(super) fn compose_add_colonyrpcmsg(
    colony: core::Colony,
    prvkey: String,
) -> std::string::String {
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
        Err(err) => return Err(RPCError::new(&err.to_string())),
    };

    let body = res.text().await;
    let body = match body {
        Ok(body) => body,
        Err(err) => return Err(RPCError::new(&err.to_string())),
    };

    let rpc_reply: RPCReplyMsg = serde_json::from_str(body.as_str()).unwrap();

    let buf = decode(rpc_reply.payload.as_str()).unwrap();
    let s = String::from_utf8(buf).expect("");

    if rpc_reply.error {
        return Err(RPCError::new(&rpc_reply.payload));
    }

    Ok(s)
}
