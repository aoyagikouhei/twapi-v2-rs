use crate::responses::{entities::Entities, withheld::Withheld};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Users {
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub entities: Option<Entities>,
    pub id: String,
    pub location: Option<String>,
    pub name: String,
    pub pinned_tweet_id: Option<String>,
    pub profile_image_url: Option<String>,
    pub protected: Option<bool>,
    pub public_metrics: Option<PublicMetrics>,
    pub url: Option<String>,
    pub username: String,
    pub verified: Option<bool>,
    pub verified_type: Option<String>,
    pub withheld: Option<Withheld>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PublicMetrics {
    pub followers_count: Option<i64>,
    pub following_count: Option<i64>,
    pub tweet_count: Option<i64>,
    pub listed_count: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
