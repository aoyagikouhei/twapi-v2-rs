use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OrganicMetrics {
    pub impression_count: Option<i64>,
    pub like_count: Option<i64>,
    pub quote_count: Option<i64>,
    pub reply_count: Option<i64>,
    pub retweet_count: Option<i64>,
    pub url_link_clicks: Option<i64>,
    pub user_profile_clicks: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
