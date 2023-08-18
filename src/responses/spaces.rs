use crate::responses::{topics::Topics};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

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
    pub topic_ids: Option<Vec<String>>, 
    pub topics: Option<Vec<Topics>>, 
    pub title: Option<String>, 
    pub updated_at: Option<DateTime<Utc>>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Spaces {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.topics.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true);
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
    fn default() -> Self { Self::Live }
}
