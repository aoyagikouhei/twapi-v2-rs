use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum TweetFields {
    Attachments,
    AuthorId,
    ContextAnnotations,
    ConversationId,
    CreatedAt,
    EditControls,
    Entities,
    Geo,
    Id,
    InReplyToUserId,
    Lang,
    NonPublicMetrics,
    PublicMetrics,
    OrganicMetrics,
    PromotedMetrics,
    PossiblySensitive,
    ReferencedTweets,
    ReplySettings,
    Source,
    Text,
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
