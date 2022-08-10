extern crate base64;
use crate::colonieslib::core;
use crate::cryptolib::crypto;
use base64::{decode, encode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct RPCError {
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
pub struct RPCMsg {
    pub signature: String,
    pub payloadtype: String,
    pub payload: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddColonyRPCMsg {
    pub colony: core::Colony,
    pub msgtype: String,
}

pub fn compose_add_colonyrpcmsg(
    colony: core::Colony,
    prvkey: String,
) -> Result<std::string::String, serde_json::Error> {
    let payloadtype = "addcolonymsg";
    let add_colony_rpcmsg = AddColonyRPCMsg {
        colony: colony.clone(),
        msgtype: payloadtype.to_owned(),
    };
    let payload = serde_json::to_string(&add_colony_rpcmsg);
    let payload = match payload {
        Ok(str) => str,
        Err(err) => panic!("{err}"),
    };
    let rpcmsg = compose_rpcmsg(
        payloadtype.to_owned(),
        payload.to_owned(),
        prvkey.to_owned(),
    );

    serde_json::to_string(&rpcmsg)
}

pub fn compose_rpcmsg(payloadtype: String, payload: String, prvkey: String) -> RPCMsg {
    let payload_base64 = base64::encode(payload.as_bytes());
    let signature = crypto::gen_signature(&payload_base64, &prvkey);
    RPCMsg {
        payload: payload_base64,
        payloadtype: payloadtype,
        signature: signature,
    }
}

pub async fn send_rpcmsg(msg: String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:50080/api")
        .body(msg)
        .send()
        .await?;

    let body = res.text().await?;

    println!("{:?}", body);

    Ok(())
}
