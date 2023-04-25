use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Errors {
    pub detail: Option<String>,
    pub parameter: Option<String>,
    pub resource_id: Option<String>,
    pub resource_type: Option<String>,
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
