use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Meta {
    pub result_count: i64, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_token: Option<String>, 
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
