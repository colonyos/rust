#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const PENDING: i32 = 0;
pub const APPROVED: i32 = 1;
pub const REJECTED: i32 = 2;

pub const IN: i32 = 0;
pub const OUT: i32 = 1;
pub const ERR: i32 = 2;
pub const ENV: i32 = 4;

pub const WAITING: i32 = 0;
pub const RUNNING: i32 = 1;
pub const SUCCESS: i32 = 2;
pub const FAILED: i32 = 3;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Failure {
    pub status: i32,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Colony {
    pub colonyid: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Runtime {
    pub runtimeid: String,
    pub runtimetype: String,
    pub name: String,
    pub colonyid: String,
    pub cpu: String,
    pub cores: i32,
    pub mem: i32,
    pub gpu: String,
    pub gpus: i32,
    pub state: i32,
    pub commissiontime: String,
    pub lastheardfromtime: String,
}

impl Runtime {
    pub fn new(name: &str, runtimeid: &str, runtimetype: &str, colonyid: &str) -> Runtime {
        Runtime {
            runtimeid: runtimeid.to_owned(),
            runtimetype: runtimetype.to_owned(),
            name: name.to_owned(),
            colonyid: colonyid.to_owned(),
            cpu: "".to_owned(),
            cores: 0,
            mem: 0,
            gpu: "".to_owned(),
            gpus: 0,
            state: 0,
            commissiontime: "2022-08-08T10:22:25.819199495+02:00".to_owned(),
            lastheardfromtime: "2022-08-08T10:22:25.819199495+02:00".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conditions {
    pub colonyid: String,
    pub runtimeids: Vec<String>,
    pub runtimetype: String,
    pub dependencies: Vec<String>,
}

impl Conditions {
    pub fn new(colonyid: &str, runtimetype: &str) -> Conditions {
        Conditions {
            colonyid: colonyid.to_owned(),
            runtimeids: Vec::new(),
            runtimetype: runtimetype.to_owned(),
            dependencies: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessSpec {
    pub name: String,
    pub func: String,
    pub args: Vec<String>,
    pub priority: i32,
    pub maxwaittime: i32,
    pub maxexectime: i32,
    pub maxretries: i32,
    pub conditions: Conditions,
    pub env: HashMap<String, String>,
}

impl ProcessSpec {
    pub fn new(
        name: &str,
        func: &str,
        args: Vec<String>,
        maxwaittime: i32,
        maxexectime: i32,
        maxretries: i32,
        conditions: Conditions,
        env: HashMap<String, String>,
    ) -> ProcessSpec {
        ProcessSpec {
            name: name.to_owned(),
            func: func.to_owned(),
            args: args,
            priority: -1,
            maxwaittime: maxwaittime,
            maxexectime: maxexectime,
            maxretries: maxretries,
            conditions: conditions,
            env: env,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attribute {
    pub attributeid: String,
    pub targetid: String,
    pub targetcolonyid: String,
    pub targetprocessgraphid: String,
    pub attributetype: i32,
    pub key: String,
    pub value: String,
}

impl Attribute {
    pub fn new(colonyid: &str, processid: &str, key: &str, value: &str) -> Attribute {
        Attribute {
            attributeid: "".to_owned(),
            targetid: processid.to_owned(),
            targetcolonyid: colonyid.to_owned(),
            targetprocessgraphid: "".to_owned(),
            attributetype: OUT,
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Process {
    pub processid: String,
    pub assignedruntimeid: String,
    pub isassigned: bool,
    pub state: i32,
    pub submissiontime: String,
    pub starttime: String,
    pub endtime: String,
    pub waitdeadline: String,
    pub execdeadline: String,
    pub retries: i32,
    pub attributes: Vec<Attribute>,
    pub spec: ProcessSpec,
    pub waitforparents: bool,
    pub parents: Vec<String>,
    pub children: Vec<String>,
    pub processgraphid: String,
    #[serde(rename = "in")]
    pub input: Vec<String>,
    #[serde(rename = "out")]
    pub output: Vec<String>,
    pub errors: Vec<String>,
}
