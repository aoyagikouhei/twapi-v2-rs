use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Options {
    pub position: Option<i64>,
    pub label: Option<String>,
    pub votes: Option<i64>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
