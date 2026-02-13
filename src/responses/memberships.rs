use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Memberships {
    pub id: String, 
    pub name: String, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_count: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
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
