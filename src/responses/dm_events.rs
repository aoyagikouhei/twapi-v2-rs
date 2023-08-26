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
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl DmEvents {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .attachments
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .referenced_tweets
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("DmEvents {:?}", self.extra);
        }
        res
    }
}
