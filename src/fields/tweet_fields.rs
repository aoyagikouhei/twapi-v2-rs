use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum TweetFields {
    #[serde(rename = "attachments")]
    Attachments,
    #[serde(rename = "author_id")]
    AuthorId,
    #[serde(rename = "context_annotations")]
    ContextAnnotations,
    #[serde(rename = "conversation_id")]
    ConversationId,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "edit_controls")]
    EditControls,
    #[serde(rename = "entities")]
    Entities,
    #[serde(rename = "geo")]
    Geo,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "in_reply_to_user_id")]
    InReplyToUserId,
    #[serde(rename = "lang")]
    Lang,
    #[serde(rename = "non_public_metrics")]
    NonPublicMetrics,
    #[serde(rename = "public_metrics")]
    PublicMetrics,
    #[serde(rename = "organic_metrics")]
    OrganicMetrics,
    #[serde(rename = "promoted_metrics")]
    PromotedMetrics,
    #[serde(rename = "possibly_sensitive")]
    PossiblySensitive,
    #[serde(rename = "referenced_tweets")]
    ReferencedTweets,
    #[serde(rename = "reply_settings")]
    ReplySettings,
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "withheld")]
    Withheld,
}

impl TweetFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Attachments);
        result.insert(Self::AuthorId);
        result.insert(Self::ContextAnnotations);
        result.insert(Self::ConversationId);
        result.insert(Self::CreatedAt);
        result.insert(Self::EditControls);
        result.insert(Self::Entities);
        result.insert(Self::Geo);
        result.insert(Self::Id);
        result.insert(Self::InReplyToUserId);
        result.insert(Self::Lang);
        result.insert(Self::NonPublicMetrics);
        result.insert(Self::PublicMetrics);
        result.insert(Self::OrganicMetrics);
        result.insert(Self::PromotedMetrics);
        result.insert(Self::PossiblySensitive);
        result.insert(Self::ReferencedTweets);
        result.insert(Self::ReplySettings);
        result.insert(Self::Source);
        result.insert(Self::Text);
        result.insert(Self::Withheld);
        result
    }

    pub fn organic() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Attachments);
        result.insert(Self::AuthorId);
        result.insert(Self::ContextAnnotations);
        result.insert(Self::ConversationId);
        result.insert(Self::CreatedAt);
        result.insert(Self::EditControls);
        result.insert(Self::Entities);
        result.insert(Self::Geo);
        result.insert(Self::Id);
        result.insert(Self::InReplyToUserId);
        result.insert(Self::Lang);
        result.insert(Self::NonPublicMetrics);
        result.insert(Self::PublicMetrics);
        result.insert(Self::OrganicMetrics);
        result.insert(Self::PossiblySensitive);
        result.insert(Self::ReferencedTweets);
        result.insert(Self::ReplySettings);
        result.insert(Self::Source);
        result.insert(Self::Text);
        result.insert(Self::Withheld);
        result
    }

    pub fn open() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Attachments);
        result.insert(Self::AuthorId);
        result.insert(Self::ContextAnnotations);
        result.insert(Self::ConversationId);
        result.insert(Self::CreatedAt);
        result.insert(Self::EditControls);
        result.insert(Self::Entities);
        result.insert(Self::Geo);
        result.insert(Self::Id);
        result.insert(Self::InReplyToUserId);
        result.insert(Self::Lang);
        result.insert(Self::PublicMetrics);
        result.insert(Self::PossiblySensitive);
        result.insert(Self::ReferencedTweets);
        result.insert(Self::ReplySettings);
        result.insert(Self::Source);
        result.insert(Self::Text);
        result.insert(Self::Withheld);
        result
    }
}

impl std::fmt::Display for TweetFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Attachments => write!(f, "attachments"),
            Self::AuthorId => write!(f, "author_id"),
            Self::ContextAnnotations => write!(f, "context_annotations"),
            Self::ConversationId => write!(f, "conversation_id"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::EditControls => write!(f, "edit_controls"),
            Self::Entities => write!(f, "entities"),
            Self::Geo => write!(f, "geo"),
            Self::Id => write!(f, "id"),
            Self::InReplyToUserId => write!(f, "in_reply_to_user_id"),
            Self::Lang => write!(f, "lang"),
            Self::NonPublicMetrics => write!(f, "non_public_metrics"),
            Self::PublicMetrics => write!(f, "public_metrics"),
            Self::OrganicMetrics => write!(f, "organic_metrics"),
            Self::PromotedMetrics => write!(f, "promoted_metrics"),
            Self::PossiblySensitive => write!(f, "possibly_sensitive"),
            Self::ReferencedTweets => write!(f, "referenced_tweets"),
            Self::ReplySettings => write!(f, "reply_settings"),
            Self::Source => write!(f, "source"),
            Self::Text => write!(f, "text"),
            Self::Withheld => write!(f, "withheld"),
        }
    }
}

impl Default for TweetFields {
    fn default() -> Self {
        Self::Attachments
    }
}
