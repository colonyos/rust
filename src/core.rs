//! Core types for ColonyOS SDK

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

// ============== Constants ==============

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

// ============== Error Types ==============

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Failure {
    pub status: i32,
    pub message: String,
}

// ============== Colony ==============

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Colony {
    pub colonyid: String,
    pub name: String,
}

impl Colony {
    pub fn new(colonyid: &str, name: &str) -> Colony {
        Colony {
            colonyid: colonyid.to_owned(),
            name: name.to_owned(),
        }
    }
}

// ============== Executor ==============

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Software {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, rename = "type", deserialize_with = "deserialize_null_default")]
    pub software_type: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Hardware {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub model: String,
    #[serde(default)]
    pub nodes: i32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub cpu: String,
    #[serde(default)]
    pub cores: i32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mem: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub storage: String,
    #[serde(default)]
    pub gpu: GPU,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub platform: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub architecture: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub network: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Capabilities {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub hardware: Vec<Hardware>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub software: Vec<Software>,
}

impl Capabilities {
    pub fn is_empty(&self) -> bool {
        self.hardware.is_empty() && self.software.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Project {
    #[serde(default)]
    pub allocatedcpu: i64,
    #[serde(default)]
    pub usedcpu: i64,
    #[serde(default)]
    pub allocatedgpu: i64,
    #[serde(default)]
    pub usedgpu: i64,
    #[serde(default)]
    pub allocatedstorage: i64,
    #[serde(default)]
    pub usedstorage: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Allocations {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub projects: HashMap<String, Project>,
}

impl Allocations {
    pub fn is_empty(&self) -> bool {
        self.projects.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Executor {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub executorid: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub executortype: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub executorname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub colonyname: String,
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub state: i32,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub requirefuncreg: bool,
    #[serde(default, deserialize_with = "deserialize_null_default", skip_serializing_if = "String::is_empty")]
    pub commissiontime: String,
    #[serde(default, deserialize_with = "deserialize_null_default", skip_serializing_if = "String::is_empty")]
    pub lastheardfromtime: String,
    #[serde(default, deserialize_with = "deserialize_null_default", skip_serializing_if = "String::is_empty")]
    pub locationname: String,
    #[serde(default, deserialize_with = "deserialize_null_default", skip_serializing_if = "Capabilities::is_empty")]
    pub capabilities: Capabilities,
    #[serde(default, deserialize_with = "deserialize_null_default", skip_serializing_if = "Allocations::is_empty")]
    pub allocations: Allocations,
    #[serde(default, deserialize_with = "deserialize_null_default", skip_serializing_if = "String::is_empty")]
    pub blueprintid: String,
    #[serde(default, skip_serializing_if = "is_zero_i64")]
    pub blueprintgen: i64,
}

fn is_zero_i32(v: &i32) -> bool { *v == 0 }
fn is_zero_i64(v: &i64) -> bool { *v == 0 }

impl Executor {
    pub fn new(name: &str, executorid: &str, executortype: &str, colonyname: &str) -> Executor {
        Executor {
            executorid: executorid.to_owned(),
            executortype: executortype.to_owned(),
            executorname: name.to_owned(),
            colonyname: colonyname.to_owned(),
            state: 0,
            requirefuncreg: false,
            commissiontime: String::new(),
            lastheardfromtime: String::new(),
            locationname: String::new(),
            capabilities: Capabilities::default(),
            allocations: Allocations::default(),
            blueprintid: String::new(),
            blueprintgen: 0,
        }
    }
}

// ============== Function Spec ==============

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GPU {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mem: String,
    #[serde(default)]
    pub count: i32,
    #[serde(default)]
    pub nodecount: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conditions {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub colonyname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub executornames: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub executortype: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub nodes: i32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub cpu: String,
    #[serde(default)]
    pub processes: i32,
    #[serde(default, rename = "processes-per-node")]
    pub processes_per_node: i32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mem: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub storage: String,
    #[serde(default)]
    pub gpu: GPU,
    #[serde(default)]
    pub walltime: i64,
}

impl Conditions {
    pub fn new(colonyname: &str, executortype: &str) -> Conditions {
        Conditions {
            colonyname: colonyname.to_owned(),
            executornames: Vec::new(),
            executortype: executortype.to_owned(),
            dependencies: Vec::new(),
            nodes: 0,
            cpu: String::new(),
            processes: 0,
            processes_per_node: 0,
            mem: String::new(),
            storage: String::new(),
            gpu: GPU::default(),
            walltime: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Filesystem {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mount: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub snapshots: Vec<SnapshotMount>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
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
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub nodename: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub funcname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub args: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub kwargs: HashMap<String, Value>,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub maxwaittime: i32,
    #[serde(default)]
    pub maxexectime: i32,
    #[serde(default)]
    pub maxretries: i32,
    pub conditions: Conditions,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub label: String,
    #[serde(default)]
    pub fs: Filesystem,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub env: HashMap<String, String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub channels: Vec<String>,
}

impl FunctionSpec {
    pub fn new(
        funcname: &str,
        executortype: &str,
        colonyname: &str,
    ) -> FunctionSpec {
        FunctionSpec {
            nodename: String::new(),
            funcname: funcname.to_owned(),
            args: Vec::new(),
            kwargs: HashMap::new(),
            priority: 0,
            maxwaittime: 0,
            maxexectime: 0,
            maxretries: 0,
            conditions: Conditions::new(colonyname, executortype),
            label: String::new(),
            fs: Filesystem::default(),
            env: HashMap::new(),
            channels: Vec::new(),
        }
    }
}

// ============== Attribute ==============

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attribute {
    #[serde(default)]
    pub attributeid: String,
    pub targetid: String,
    pub targetcolonyname: String,
    #[serde(default)]
    pub targetprocessgraphid: String,
    #[serde(default)]
    pub attributetype: i32,
    pub key: String,
    pub value: String,
}

impl Attribute {
    pub fn new(colonyname: &str, processid: &str, key: &str, value: &str) -> Attribute {
        Attribute {
            attributeid: String::new(),
            targetid: processid.to_owned(),
            targetcolonyname: colonyname.to_owned(),
            targetprocessgraphid: String::new(),
            attributetype: OUT,
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }
}

// ============== Process ==============

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Process {
    pub processid: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub initiatorid: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub initiatorname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub assignedexecutorid: String,
    #[serde(default)]
    pub isassigned: bool,
    #[serde(default)]
    pub state: i32,
    #[serde(default)]
    pub prioritytime: i64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub submissiontime: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub starttime: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub endtime: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub waitdeadline: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub execdeadline: String,
    #[serde(default)]
    pub retries: i32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub attributes: Vec<Attribute>,
    pub spec: FunctionSpec,
    #[serde(default)]
    pub waitforparents: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub parents: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub children: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub processgraphid: String,
    #[serde(default, rename = "in", deserialize_with = "deserialize_null_default")]
    pub input: Vec<String>,
    #[serde(default, rename = "out", deserialize_with = "deserialize_null_default")]
    pub output: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub errors: Vec<String>,
}

// ============== Process Graph (Workflow) ==============

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessGraph {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub processgraphid: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub colonyname: String,
    #[serde(default)]
    pub state: i32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub rootprocessids: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub processids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowSpec {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub colonyname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub functionspecs: Vec<FunctionSpec>,
}

// ============== Log ==============

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Log {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub processid: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub colonyname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub executorname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub message: String,
    #[serde(default)]
    pub timestamp: i64,
}

// ============== Channel ==============

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ChannelEntry {
    #[serde(default)]
    pub sequence: i64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub payload: String,  // Base64 encoded payload
    #[serde(default, rename = "type", deserialize_with = "deserialize_null_default")]
    pub msgtype: String,
    #[serde(default)]
    pub inreplyto: i64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub timestamp: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub senderid: String,
}

impl ChannelEntry {
    /// Returns the payload decoded from base64 as a UTF-8 string.
    pub fn payload_as_string(&self) -> String {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        match STANDARD.decode(&self.payload) {
            Ok(bytes) => String::from_utf8(bytes).unwrap_or_default(),
            Err(_) => self.payload.clone(), // Return as-is if not base64
        }
    }

    /// Returns the raw payload bytes decoded from base64.
    pub fn payload_bytes(&self) -> Vec<u8> {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        STANDARD.decode(&self.payload).unwrap_or_default()
    }
}

// ============== Statistics ==============

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Statistics {
    #[serde(default)]
    pub colonies: i64,
    #[serde(default)]
    pub executors: i64,
    #[serde(default)]
    pub waitingprocesses: i64,
    #[serde(default)]
    pub runningprocesses: i64,
    #[serde(default)]
    pub successfulprocesses: i64,
    #[serde(default)]
    pub failedprocesses: i64,
    #[serde(default)]
    pub waitingworkflows: i64,
    #[serde(default)]
    pub runningworkflows: i64,
    #[serde(default)]
    pub successfulworkflows: i64,
    #[serde(default)]
    pub failedworkflows: i64,
}

// ============== Blueprint ==============

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BlueprintDefinition {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub colonyname: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub executortype: String,
    #[serde(default)]
    pub specschema: HashMap<String, Value>,
    #[serde(default)]
    pub statusschema: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BlueprintMetadata {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub colonyname: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BlueprintHandler {
    #[serde(default)]
    pub executortype: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Blueprint {
    #[serde(default)]
    pub blueprintid: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub metadata: BlueprintMetadata,
    #[serde(default)]
    pub handler: BlueprintHandler,
    #[serde(default)]
    pub spec: HashMap<String, Value>,
    #[serde(default)]
    pub status: HashMap<String, Value>,
    #[serde(default)]
    pub generation: i64,
    #[serde(default)]
    pub reconciledgeneration: i64,
    #[serde(default)]
    pub lastreconciled: String,
}

// ============== Function ==============

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function {
    #[serde(default)]
    pub functionid: String,
    #[serde(default)]
    pub executorname: String,
    #[serde(default)]
    pub executortype: String,
    #[serde(default)]
    pub colonyname: String,
    pub funcname: String,
    #[serde(default)]
    pub counter: i64,
    #[serde(default)]
    pub minwaittime: f64,
    #[serde(default)]
    pub maxwaittime: f64,
    #[serde(default)]
    pub minexectime: f64,
    #[serde(default)]
    pub maxexectime: f64,
    #[serde(default)]
    pub avgwaittime: f64,
    #[serde(default)]
    pub avgexectime: f64,
}
