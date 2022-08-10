use serde::{Deserialize, Serialize};

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
