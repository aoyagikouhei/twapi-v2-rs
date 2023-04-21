use crate::responses::{entities::Entities, public_metrics::PublicMetrics};
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
    pub username: String,
    pub verified: Option<bool>,
    pub verified_type: Option<String>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
