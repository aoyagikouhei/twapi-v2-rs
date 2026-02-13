use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct OrganicMetrics {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_link_clicks: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_clicks: Option<i64>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl OrganicMetrics {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("OrganicMetrics {:?}", self.extra);
        }
        res
    }
}
