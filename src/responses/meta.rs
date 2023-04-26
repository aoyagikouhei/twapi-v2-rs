use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Meta {
    pub result_count: i64,
    pub newest_id: String,
    pub oldest_id: String,
    pub next_token: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Meta {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Meta {:?}", self.extra);
        }
        res
    }
}
