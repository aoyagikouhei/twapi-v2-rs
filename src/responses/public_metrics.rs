use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PublicMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmark_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retweet_count: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl PublicMetrics {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("PublicMetrics {:?}", self.extra);
        }
        res
    }
}
