use crate::responses::topics::Topics;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Spaces {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ticketed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    pub state: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Topics>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Spaces {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .topics
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Spaces {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum State {
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "scheduled")]
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
