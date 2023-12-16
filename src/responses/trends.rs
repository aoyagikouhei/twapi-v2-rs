use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Trends {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trend_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweet_count: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Trends {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Trends {:?}", self.extra);
        }
        res
    }
}
