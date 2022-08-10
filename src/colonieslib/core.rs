use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Colony {
    pub colonyid: String,
    pub name: String,
}

pub fn marshall_colony_json(colony: &Colony) -> Result<std::string::String, serde_json::Error> {
    serde_json::to_string(colony)
}

pub fn unmarshall_colony_json(json: String) -> Result<Colony, serde_json::Error> {
    serde_json::from_str(json.as_str())
}
