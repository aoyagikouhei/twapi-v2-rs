use crate::responses::{attachments::Attachments, referenced_tweets::ReferencedTweets};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DmEvents {
    pub id: Option<String>,
    pub text: Option<String>,
    pub event_type: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub sender_id: Option<String>,
    pub dm_conversation_id: Option<String>,
    pub attachments: Option<Attachments>,
    pub referenced_tweets: Option<Vec<ReferencedTweets>>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
