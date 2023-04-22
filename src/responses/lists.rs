use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Lists {
    pub id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub private: Option<bool>,
    pub follower_count: Option<i64>,
    pub member_count: Option<i64>,
    pub owner_id: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
