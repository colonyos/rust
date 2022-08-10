use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Colony {
    pub colonyid: String,
    pub name: String,
}
