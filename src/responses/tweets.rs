use crate::responses::{
    attachments::Attachments, context_annotations::ContextAnnotations, edit_controls::EditControls,
    entities::Entities, geo::Geo, non_public_metrics::NonPublicMetrics,
    organic_metrics::OrganicMetrics, promoted_metrics::PromotedMetrics,
    public_metrics::PublicMetrics, referenced_tweets::ReferencedTweets, withheld::Withheld,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tweets {
    pub attachments: Option<Attachments>,
    pub author_id: Option<String>,
    pub context_annotations: Option<Vec<ContextAnnotations>>,
    pub conversation_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub edit_controls: Option<EditControls>,
    pub edit_history_tweet_ids: Vec<String>,
    pub entities: Option<Entities>,
    pub geo: Option<Geo>,
    pub id: String,
    pub in_reply_to_user_id: Option<String>,
    pub lang: Option<String>,
    pub possibly_sensitive: Option<bool>,
    pub public_metrics: Option<PublicMetrics>,
    pub non_public_metrics: Option<NonPublicMetrics>,
    pub organic_metrics: Option<OrganicMetrics>,
    pub promoted_metrics: Option<PromotedMetrics>,
    pub reply_settings: Option<String>,
    pub referenced_tweets: Option<Vec<ReferencedTweets>>,
    pub text: String,
    pub withheld: Option<Withheld>,
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
