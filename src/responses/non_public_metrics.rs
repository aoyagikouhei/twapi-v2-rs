use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NonPublicMetrics {
    pub impression_count: Option<i64>,
    pub url_link_clicks: Option<i64>,
    pub user_profile_clicks: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl NonPublicMetrics {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("NonPublicMetrics {:?}", self.extra);
        }
        res
    }
}
