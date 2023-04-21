use crate::responses::{
    attachments::Attachments, entities::Entities, public_metrics::PublicMetrics,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tweets {
    pub attachments: Option<Attachments>,
    pub author_id: Option<String>,
    pub conversation_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub edit_controls: Option<Attachments>,
    pub edit_history_tweet_ids: Option<Vec<String>>,
    pub entities: Option<Entities>,
    pub id: Option<String>,
    pub lang: Option<String>,
    pub possibly_sensitive: Option<bool>,
    pub public_metrics: Option<PublicMetrics>,
    pub reply_settings: Option<String>,
    pub text: Option<String>,
}
