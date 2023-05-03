use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MetaCount {
    pub total_tweet_count: i64,
    pub next_token: Option<String>,
    pub previous_token: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl MetaCount {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("MetaCount {:?}", self.extra);
        }
        res
    }
}
