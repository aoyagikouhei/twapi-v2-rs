use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Meta {
    pub count: i64,
    pub newest_id: i64,
    pub oldest_id: i64,
    pub next_token: Option<String>,
    pub previous_token: Option<String>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
