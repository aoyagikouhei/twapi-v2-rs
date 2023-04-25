use crate::responses::topics::Topics;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Spaces {
    pub id: String,
    pub host_ids: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub creator_id: Option<String>,
    pub ended_at: Option<DateTime<Utc>>,
    pub lang: Option<String>,
    pub is_ticketed: Option<bool>,
    pub invited_user_ids: Option<Vec<String>>,
    pub participant_count: Option<i64>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub speaker_ids: Option<Vec<String>>,
    pub started_at: Option<DateTime<Utc>>,
    pub state: State,
    pub subscriber_count: Option<i64>,
    pub topic_ids: Option<String>,
    pub topics: Option<Vec<Topics>>,
    pub title: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum State {
    Live,
    Scheduled,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Live => write!(f, "live"),
            Self::Scheduled => write!(f, "scheduled"),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Live
    }
}
