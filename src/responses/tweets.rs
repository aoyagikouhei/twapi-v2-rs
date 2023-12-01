use crate::responses::{
    attachments::Attachments, context_annotations::ContextAnnotations, edit_controls::EditControls,
    entities::Entities, geo::Geo, non_public_metrics::NonPublicMetrics, note_tweet::NoteTweet,
    organic_metrics::OrganicMetrics, promoted_metrics::PromotedMetrics,
    public_metrics::PublicMetrics, referenced_tweets::ReferencedTweets, withheld::Withheld,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tweets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_annotations: Option<Vec<ContextAnnotations>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_controls: Option<EditControls>,
    pub edit_history_tweet_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Entities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possibly_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<PublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_public_metrics: Option<NonPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_tweet: Option<NoteTweet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organic_metrics: Option<OrganicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoted_metrics: Option<PromotedMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_tweets: Option<Vec<ReferencedTweets>>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withheld: Option<Withheld>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Tweets {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .attachments
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .context_annotations
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .edit_controls
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .entities
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .geo
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .public_metrics
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .non_public_metrics
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .note_tweet
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .organic_metrics
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .promoted_metrics
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .referenced_tweets
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .withheld
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Tweets {:?}", self.extra);
        }
        res
    }
}
