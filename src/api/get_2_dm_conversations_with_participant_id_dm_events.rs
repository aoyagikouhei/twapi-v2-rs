use super::{execute_twitter, TwitterResult};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/dm_conversations/with/:participant_id/dm_events";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum DmEventFields {
    Id,
    Text,
    EventType,
    CreatedAt,
    DmConversationId,
    SenderId,
    ParticipantIds,
    ReferencedTweets,
    Attachments,
}

impl DmEventFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Id);
        result.insert(Self::Text);
        result.insert(Self::EventType);
        result.insert(Self::CreatedAt);
        result.insert(Self::DmConversationId);
        result.insert(Self::SenderId);
        result.insert(Self::ParticipantIds);
        result.insert(Self::ReferencedTweets);
        result.insert(Self::Attachments);
        result
    }
}

impl std::fmt::Display for DmEventFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Text => write!(f, "text"),
            Self::EventType => write!(f, "event_type"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::DmConversationId => write!(f, "dm_conversation_id"),
            Self::SenderId => write!(f, "sender_id"),
            Self::ParticipantIds => write!(f, "participant_ids"),
            Self::ReferencedTweets => write!(f, "referenced_tweets"),
            Self::Attachments => write!(f, "attachments"),
        }
    }
}

impl Default for DmEventFields {
    fn default() -> Self {
        Self::Id
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum EventTypes {
    Messagecreate,
    Participantsjoin,
    Participantsleave,
}

impl std::fmt::Display for EventTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Messagecreate => write!(f, "MessageCreate"),
            Self::Participantsjoin => write!(f, "ParticipantsJoin"),
            Self::Participantsleave => write!(f, "ParticipantsLeave"),
        }
    }
}

impl Default for EventTypes {
    fn default() -> Self {
        Self::Messagecreate
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    AttachmentsMediaKeys,
    ReferencedTweetsId,
    SenderId,
    ParticipantIds,
}

impl Expansions {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::AttachmentsMediaKeys);
        result.insert(Self::ReferencedTweetsId);
        result.insert(Self::SenderId);
        result.insert(Self::ParticipantIds);
        result
    }
}

impl std::fmt::Display for Expansions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AttachmentsMediaKeys => write!(f, "attachments.media_keys"),
            Self::ReferencedTweetsId => write!(f, "referenced_tweets.id"),
            Self::SenderId => write!(f, "sender_id"),
            Self::ParticipantIds => write!(f, "participant_ids"),
        }
    }
}

impl Default for Expansions {
    fn default() -> Self {
        Self::AttachmentsMediaKeys
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum MediaFields {
    DurationMs,
    Height,
    MediaKey,
    PreviewImageUrl,
    Type,
    Url,
    Width,
    PublicMetrics,
    AltText,
    Variants,
}

impl MediaFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::DurationMs);
        result.insert(Self::Height);
        result.insert(Self::MediaKey);
        result.insert(Self::PreviewImageUrl);
        result.insert(Self::Type);
        result.insert(Self::Url);
        result.insert(Self::Width);
        result.insert(Self::PublicMetrics);
        result.insert(Self::AltText);
        result.insert(Self::Variants);
        result
    }
}

impl std::fmt::Display for MediaFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DurationMs => write!(f, "duration_ms"),
            Self::Height => write!(f, "height"),
            Self::MediaKey => write!(f, "media_key"),
            Self::PreviewImageUrl => write!(f, "preview_image_url"),
            Self::Type => write!(f, "type"),
            Self::Url => write!(f, "url"),
            Self::Width => write!(f, "width"),
            Self::PublicMetrics => write!(f, "public_metrics"),
            Self::AltText => write!(f, "alt_text"),
            Self::Variants => write!(f, "variants"),
        }
    }
}

impl Default for MediaFields {
    fn default() -> Self {
        Self::DurationMs
    }
}

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
    PublicMetrics,
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
            Self::PublicMetrics => write!(f, "public_metrics"),
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

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum UserFields {
    CreatedAt,
    Description,
    Entities,
    Id,
    Location,
    Name,
    PinnedTweetId,
    ProfileImageUrl,
    Protected,
    PublicMetrics,
    Url,
    Username,
    Verified,
    Withheld,
}

impl UserFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::CreatedAt);
        result.insert(Self::Description);
        result.insert(Self::Entities);
        result.insert(Self::Id);
        result.insert(Self::Location);
        result.insert(Self::Name);
        result.insert(Self::PinnedTweetId);
        result.insert(Self::ProfileImageUrl);
        result.insert(Self::Protected);
        result.insert(Self::PublicMetrics);
        result.insert(Self::Url);
        result.insert(Self::Username);
        result.insert(Self::Verified);
        result.insert(Self::Withheld);
        result
    }
}

impl std::fmt::Display for UserFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::Description => write!(f, "description"),
            Self::Entities => write!(f, "entities"),
            Self::Id => write!(f, "id"),
            Self::Location => write!(f, "location"),
            Self::Name => write!(f, "name"),
            Self::PinnedTweetId => write!(f, "pinned_tweet_id"),
            Self::ProfileImageUrl => write!(f, "profile_image_url"),
            Self::Protected => write!(f, "protected"),
            Self::PublicMetrics => write!(f, "public_metrics"),
            Self::Url => write!(f, "url"),
            Self::Username => write!(f, "username"),
            Self::Verified => write!(f, "verified"),
            Self::Withheld => write!(f, "withheld"),
        }
    }
}

impl Default for UserFields {
    fn default() -> Self {
        Self::CreatedAt
    }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    participant_id: String,
    dm_event_fields: Option<HashSet<DmEventFields>>,
    event_types: Option<EventTypes>,
    expansions: Option<HashSet<Expansions>>,
    max_results: Option<usize>,
    media_fields: Option<HashSet<MediaFields>>,
    pagination_token: Option<String>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(bearer_code: &str, participant_id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            participant_id: participant_id.to_owned(),
            ..Default::default()
        }
    }

    pub fn dm_event_fields(mut self, value: HashSet<DmEventFields>) -> Self {
        self.dm_event_fields = Some(value);
        self
    }

    pub fn event_types(mut self, value: EventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn expansions(mut self, value: HashSet<Expansions>) -> Self {
        self.expansions = Some(value);
        self
    }

    pub fn max_results(mut self, value: usize) -> Self {
        self.max_results = Some(value);
        self
    }

    pub fn media_fields(mut self, value: HashSet<MediaFields>) -> Self {
        self.media_fields = Some(value);
        self
    }

    pub fn pagination_token(mut self, value: &str) -> Self {
        self.pagination_token = Some(value.to_owned());
        self
    }

    pub fn tweet_fields(mut self, value: HashSet<TweetFields>) -> Self {
        self.tweet_fields = Some(value);
        self
    }

    pub fn user_fields(mut self, value: HashSet<UserFields>) -> Self {
        self.user_fields = Some(value);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(dm_event_fields) = self.dm_event_fields {
            query_parameters.push(("dm_event.fields", dm_event_fields.iter().join(",")));
        }
        if let Some(event_types) = self.event_types {
            query_parameters.push(("event_types", event_types.to_string()));
        }
        if let Some(expansions) = self.expansions {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(max_results) = self.max_results {
            query_parameters.push(("max_results", max_results.to_string()));
        }
        if let Some(media_fields) = self.media_fields {
            query_parameters.push(("media.fields", media_fields.iter().join(",")));
        }
        if let Some(pagination_token) = self.pagination_token {
            query_parameters.push(("pagination_token", pagination_token));
        }
        if let Some(tweet_fields) = self.tweet_fields {
            query_parameters.push(("tweet.fields", tweet_fields.iter().join(",")));
        }
        if let Some(user_fields) = self.user_fields {
            query_parameters.push(("user.fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        client
            .get(URL.replace(":participant_id", &self.participant_id))
            .query(&query_parameters)
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}
