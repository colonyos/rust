#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

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
pub struct Function {
    pub funcname: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Location {
    pub long: f64,
    pub lat: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Executor {
    pub executorid: String,
    pub executortype: String,
    pub executorname: String,
    pub colonyname: String,
    pub state: i32,
    pub commissiontime: String,
    pub lastheardfromtime: String,
    pub location: Location,
}

impl Executor {
    pub fn new(name: &str, executorid: &str, executortype: &str, colonyname: &str) -> Executor {
        Executor {
            executorid: executorid.to_owned(),
            executortype: executortype.to_owned(),
            executorname: name.to_owned(),
            colonyname: colonyname.to_owned(),
            state: 0,
            commissiontime: "2022-08-08T10:22:25.819199495+02:00".to_owned(),
            lastheardfromtime: "2022-08-08T10:22:25.819199495+02:00".to_owned(),
            location: Location {
                long: 0.0,
                lat: 0.0,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GPU {
    pub name: String,
    pub mem: String,
    pub count: i32,
    pub nodecount: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conditions {
    pub colonyname: String,
    pub executornames: Vec<String>,
    pub executortype: String,
    pub dependencies: Vec<String>,
    pub nodes: i32,
    pub cpu: String,
    pub processes: i32,
    #[serde(rename = "processes-per-node")]
    pub processes_per_node: i32,  
    pub mem: String,
    pub storage: String,
    pub gpu: GPU,
    pub walltime: i64,
}

// impl Conditions {
//     pub fn new(colonyname: &str, executortype: &str) -> Conditions {
//         Conditions {
//             colonyname: colonyname.to_owned(),
//             executorids: Vec::new(),
//             executortype: executortype.to_owned(),
//             dependencies: Vec::new(),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Filesystem {
    pub mount: String,
    pub snapshots: Vec<SnapshotMount>,
    pub dirs: Vec<SyncDirMount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapshotMount {
    pub snapshotid: String,
    pub label: String,
    pub dir: String,
    pub keepfiles: bool,
    pub keepsnapshot: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnStart {
    pub keeplocal: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnClose {
    pub keeplocal: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConflictResolution {
    pub onstart: OnStart,
    pub onclose: OnClose,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncDirMount {
    pub label: String,
    pub dir: String,
    pub keepfiles: bool,
    pub onconflicts: ConflictResolution,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionSpec {
    pub nodename: String,
    pub funcname: String,
    pub args: Vec<String>,
    pub kwargs: HashMap<String, Value>,
    pub priority: i32,
    pub maxwaittime: i32,
    pub maxexectime: i32,
    pub maxretries: i32,
    pub conditions: Conditions,
    pub label: String,
    pub fs: Filesystem,
    pub env: HashMap<String, String>,
}

impl FunctionSpec {
    pub fn new(
        nodename: &str,
        funcname: &str,
        args: Vec<String>,
        maxwaittime: i32,
        maxexectime: i32,
        maxretries: i32,
        conditions: Conditions,
        label: &str,
        env: HashMap<String, String>,
    ) -> FunctionSpec {
        FunctionSpec {
            nodename: nodename.to_owned(),
            funcname: funcname.to_owned(),
            args: args,
            kwargs: HashMap::new(),
            priority: -1,
            maxwaittime: maxwaittime,
            maxexectime: maxexectime,
            maxretries: maxretries,
            conditions: conditions,
            label: label.to_owned(),
            fs: Filesystem {
                mount: "".to_owned(),
                snapshots: Vec::new(),
                dirs: Vec::new(),
            },
            env: env,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attribute {
    pub attributeid: String,
    pub targetid: String,
    pub targetcolonyname: String,
    pub targetprocessgraphid: String,
    pub attributetype: i32,
    pub key: String,
    pub value: String,
}

impl Attribute {
    pub fn new(colonyname: &str, processid: &str, key: &str, value: &str) -> Attribute {
        Attribute {
            attributeid: "".to_owned(),
            targetid: processid.to_owned(),
            targetcolonyname: colonyname.to_owned(),
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
    pub initiatorid: String,
    pub initiatorname: String,
    pub assignedexecutorid: String,
    pub isassigned: bool,
    pub state: i32,
    pub prioritytime: i64,
    pub submissiontime: String,
    pub starttime: String,
    pub endtime: String,
    pub waitdeadline: String,
    pub execdeadline: String,
    pub retries: i32,
    pub attributes: Vec<Attribute>,
    pub spec: FunctionSpec,
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
