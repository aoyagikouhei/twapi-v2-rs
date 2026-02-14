use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone, Default)]
pub enum TweetFields {
    #[serde(rename = "article")]
    #[default]
    Article,
    #[serde(rename = "attachments")]
    Attachments,
    #[serde(rename = "author_id")]
    AuthorId,
    #[serde(rename = "card_uri")]
    CardUri,
    #[serde(rename = "community_id")]
    CommunityId,
    #[serde(rename = "context_annotations")]
    ContextAnnotations,
    #[serde(rename = "conversation_id")]
    ConversationId,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "display_text_range")]
    DisplayTextRange,
    #[serde(rename = "edit_controls")]
    EditControls,
    #[serde(rename = "edit_history_tweet_ids")]
    EditHistoryTweetIds,
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
    #[serde(rename = "media_metadata")]
    MediaMetadata,
    #[serde(rename = "non_public_metrics")]
    NonPublicMetrics,
    #[serde(rename = "note_tweet")]
    NoteTweet,
    #[serde(rename = "organic_metrics")]
    OrganicMetrics,
    #[serde(rename = "possibly_sensitive")]
    PossiblySensitive,
    #[serde(rename = "promoted_metrics")]
    PromotedMetrics,
    #[serde(rename = "public_metrics")]
    PublicMetrics,
    #[serde(rename = "referenced_tweets")]
    ReferencedTweets,
    #[serde(rename = "reply_settings")]
    ReplySettings,
    #[serde(rename = "scopes")]
    Scopes,
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
        result.insert(Self::Article);
        result.insert(Self::Attachments);
        result.insert(Self::AuthorId);
        result.insert(Self::CardUri);
        result.insert(Self::CommunityId);
        result.insert(Self::ContextAnnotations);
        result.insert(Self::ConversationId);
        result.insert(Self::CreatedAt);
        result.insert(Self::DisplayTextRange);
        result.insert(Self::EditControls);
        result.insert(Self::EditHistoryTweetIds);
        result.insert(Self::Entities);
        result.insert(Self::Geo);
        result.insert(Self::Id);
        result.insert(Self::InReplyToUserId);
        result.insert(Self::Lang);
        result.insert(Self::MediaMetadata);
        result.insert(Self::NonPublicMetrics);
        result.insert(Self::NoteTweet);
        result.insert(Self::OrganicMetrics);
        result.insert(Self::PossiblySensitive);
        result.insert(Self::PromotedMetrics);
        result.insert(Self::PublicMetrics);
        result.insert(Self::ReferencedTweets);
        result.insert(Self::ReplySettings);
        result.insert(Self::Scopes);
        result.insert(Self::Source);
        result.insert(Self::Text);
        result.insert(Self::Withheld);
        result
    }

    pub fn organic() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Article);
        result.insert(Self::Attachments);
        result.insert(Self::AuthorId);
        result.insert(Self::CardUri);
        result.insert(Self::CommunityId);
        result.insert(Self::ContextAnnotations);
        result.insert(Self::ConversationId);
        result.insert(Self::CreatedAt);
        result.insert(Self::DisplayTextRange);
        result.insert(Self::EditControls);
        result.insert(Self::EditHistoryTweetIds);
        result.insert(Self::Entities);
        result.insert(Self::Geo);
        result.insert(Self::Id);
        result.insert(Self::InReplyToUserId);
        result.insert(Self::Lang);
        result.insert(Self::MediaMetadata);
        result.insert(Self::NonPublicMetrics);
        result.insert(Self::NoteTweet);
        result.insert(Self::OrganicMetrics);
        result.insert(Self::PossiblySensitive);
        result.insert(Self::PromotedMetrics);
        result.insert(Self::PublicMetrics);
        result.insert(Self::ReferencedTweets);
        result.insert(Self::ReplySettings);
        result.insert(Self::Scopes);
        result.insert(Self::Source);
        result.insert(Self::Text);
        result.insert(Self::Withheld);
        result
    }

    pub fn open() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Article);
        result.insert(Self::Attachments);
        result.insert(Self::AuthorId);
        result.insert(Self::CardUri);
        result.insert(Self::CommunityId);
        result.insert(Self::ContextAnnotations);
        result.insert(Self::ConversationId);
        result.insert(Self::CreatedAt);
        result.insert(Self::DisplayTextRange);
        result.insert(Self::EditControls);
        result.insert(Self::EditHistoryTweetIds);
        result.insert(Self::Entities);
        result.insert(Self::Geo);
        result.insert(Self::Id);
        result.insert(Self::InReplyToUserId);
        result.insert(Self::Lang);
        result.insert(Self::MediaMetadata);
        result.insert(Self::NoteTweet);
        result.insert(Self::PossiblySensitive);
        result.insert(Self::PublicMetrics);
        result.insert(Self::ReferencedTweets);
        result.insert(Self::ReplySettings);
        result.insert(Self::Scopes);
        result.insert(Self::Source);
        result.insert(Self::Text);
        result.insert(Self::Withheld);
        result
    }
}

impl std::fmt::Display for TweetFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Article => write!(f, "article"),
            Self::Attachments => write!(f, "attachments"),
            Self::AuthorId => write!(f, "author_id"),
            Self::CardUri => write!(f, "card_uri"),
            Self::CommunityId => write!(f, "community_id"),
            Self::ContextAnnotations => write!(f, "context_annotations"),
            Self::ConversationId => write!(f, "conversation_id"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::DisplayTextRange => write!(f, "display_text_range"),
            Self::EditControls => write!(f, "edit_controls"),
            Self::EditHistoryTweetIds => write!(f, "edit_history_tweet_ids"),
            Self::Entities => write!(f, "entities"),
            Self::Geo => write!(f, "geo"),
            Self::Id => write!(f, "id"),
            Self::InReplyToUserId => write!(f, "in_reply_to_user_id"),
            Self::Lang => write!(f, "lang"),
            Self::MediaMetadata => write!(f, "media_metadata"),
            Self::NonPublicMetrics => write!(f, "non_public_metrics"),
            Self::NoteTweet => write!(f, "note_tweet"),
            Self::OrganicMetrics => write!(f, "organic_metrics"),
            Self::PossiblySensitive => write!(f, "possibly_sensitive"),
            Self::PromotedMetrics => write!(f, "promoted_metrics"),
            Self::PublicMetrics => write!(f, "public_metrics"),
            Self::ReferencedTweets => write!(f, "referenced_tweets"),
            Self::ReplySettings => write!(f, "reply_settings"),
            Self::Scopes => write!(f, "scopes"),
            Self::Source => write!(f, "source"),
            Self::Text => write!(f, "text"),
            Self::Withheld => write!(f, "withheld"),
        }
    }
}
