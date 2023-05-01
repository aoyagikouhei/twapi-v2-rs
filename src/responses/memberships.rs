use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Memberships {
    pub id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub private: Option<bool>,
    pub follower_count: Option<i64>,
    pub member_count: Option<i64>,
    pub owner_id: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Memberships {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Memberships {:?}", self.extra);
        }
        res
    }
}
