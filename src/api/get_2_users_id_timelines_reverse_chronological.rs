use super::{execute_twitter, TwitterResult};
use chrono::prelude::*;
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/users/:id/timelines/reverse_chronological";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Exclude {
    Retweets,
    Replies,
}

impl Exclude {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Retweets);
        result.insert(Self::Replies);
        result
    }
}

impl std::fmt::Display for Exclude {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Retweets => write!(f, "retweets"),
            Self::Replies => write!(f, "replies"),
        }
    }
}

impl Default for Exclude {
    fn default() -> Self {
        Self::Retweets
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    AttachmentsPollIds,
    AttachmentsMediaKeys,
    AuthorId,
    EditHistoryTweetIds,
    EntitiesMentionsUsername,
    GeoPlaceId,
    InReplyToUserId,
    ReferencedTweetsId,
    ReferencedTweetsIdAuthorId,
}

impl Expansions {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::AttachmentsPollIds);
        result.insert(Self::AttachmentsMediaKeys);
        result.insert(Self::AuthorId);
        result.insert(Self::EditHistoryTweetIds);
        result.insert(Self::EntitiesMentionsUsername);
        result.insert(Self::GeoPlaceId);
        result.insert(Self::InReplyToUserId);
        result.insert(Self::ReferencedTweetsId);
        result.insert(Self::ReferencedTweetsIdAuthorId);
        result
    }
}

impl std::fmt::Display for Expansions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AttachmentsPollIds => write!(f, "attachments.poll_ids"),
            Self::AttachmentsMediaKeys => write!(f, "attachments.media_keys"),
            Self::AuthorId => write!(f, "author_id"),
            Self::EditHistoryTweetIds => write!(f, "edit_history_tweet_ids"),
            Self::EntitiesMentionsUsername => write!(f, "entities.mentions.username"),
            Self::GeoPlaceId => write!(f, "geo.place_id"),
            Self::InReplyToUserId => write!(f, "in_reply_to_user_id"),
            Self::ReferencedTweetsId => write!(f, "referenced_tweets.id"),
            Self::ReferencedTweetsIdAuthorId => write!(f, "referenced_tweets.id.author_id"),
        }
    }
}

impl Default for Expansions {
    fn default() -> Self {
        Self::AttachmentsPollIds
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
    NonPublicMetrics,
    OrganicMetrics,
    PromotedMetrics,
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
        result.insert(Self::NonPublicMetrics);
        result.insert(Self::OrganicMetrics);
        result.insert(Self::PromotedMetrics);
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
            Self::NonPublicMetrics => write!(f, "non_public_metrics"),
            Self::OrganicMetrics => write!(f, "organic_metrics"),
            Self::PromotedMetrics => write!(f, "promoted_metrics"),
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
pub enum PlaceFields {
    ContainedWithin,
    Country,
    CountryCode,
    FullName,
    Geo,
    Id,
    Name,
    PlaceType,
}

impl PlaceFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::ContainedWithin);
        result.insert(Self::Country);
        result.insert(Self::CountryCode);
        result.insert(Self::FullName);
        result.insert(Self::Geo);
        result.insert(Self::Id);
        result.insert(Self::Name);
        result.insert(Self::PlaceType);
        result
    }
}

impl std::fmt::Display for PlaceFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ContainedWithin => write!(f, "contained_within"),
            Self::Country => write!(f, "country"),
            Self::CountryCode => write!(f, "country_code"),
            Self::FullName => write!(f, "full_name"),
            Self::Geo => write!(f, "geo"),
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::PlaceType => write!(f, "place_type"),
        }
    }
}

impl Default for PlaceFields {
    fn default() -> Self {
        Self::ContainedWithin
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum PollFields {
    DurationMinutes,
    EndDatetime,
    Id,
    Options,
    VotingStatus,
}

impl PollFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::DurationMinutes);
        result.insert(Self::EndDatetime);
        result.insert(Self::Id);
        result.insert(Self::Options);
        result.insert(Self::VotingStatus);
        result
    }
}

impl std::fmt::Display for PollFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DurationMinutes => write!(f, "duration_minutes"),
            Self::EndDatetime => write!(f, "end_datetime"),
            Self::Id => write!(f, "id"),
            Self::Options => write!(f, "options"),
            Self::VotingStatus => write!(f, "voting_status"),
        }
    }
}

impl Default for PollFields {
    fn default() -> Self {
        Self::DurationMinutes
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
    VerifiedType,
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
        result.insert(Self::VerifiedType);
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
            Self::VerifiedType => write!(f, "verified_type"),
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
    id: String,
    end_time: Option<DateTime<Utc>>,
    exclude: Option<HashSet<Exclude>>,
    expansions: Option<HashSet<Expansions>>,
    max_results: Option<usize>,
    media_fields: Option<HashSet<MediaFields>>,
    pagination_token: Option<String>,
    place_fields: Option<HashSet<PlaceFields>>,
    poll_fields: Option<HashSet<PollFields>>,
    since_id: Option<String>,
    start_time: Option<DateTime<Utc>>,
    tweet_fields: Option<HashSet<TweetFields>>,
    until_id: Option<String>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(bearer_code: &str, id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            id: id.to_owned(),
            ..Default::default()
        }
    }

    pub fn end_time(mut self, value: DateTime<Utc>) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn exclude(mut self, value: HashSet<Exclude>) -> Self {
        self.exclude = Some(value);
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

    pub fn place_fields(mut self, value: HashSet<PlaceFields>) -> Self {
        self.place_fields = Some(value);
        self
    }

    pub fn poll_fields(mut self, value: HashSet<PollFields>) -> Self {
        self.poll_fields = Some(value);
        self
    }

    pub fn since_id(mut self, value: &str) -> Self {
        self.since_id = Some(value.to_owned());
        self
    }

    pub fn start_time(mut self, value: DateTime<Utc>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn tweet_fields(mut self, value: HashSet<TweetFields>) -> Self {
        self.tweet_fields = Some(value);
        self
    }

    pub fn until_id(mut self, value: &str) -> Self {
        self.until_id = Some(value.to_owned());
        self
    }

    pub fn user_fields(mut self, value: HashSet<UserFields>) -> Self {
        self.user_fields = Some(value);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(end_time) = self.end_time {
            query_parameters.push(("end_time", end_time.format("%Y-%m-%dT%H%M%SZ").to_string()));
        }
        if let Some(exclude) = self.exclude {
            query_parameters.push(("exclude", exclude.iter().join(",")));
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
        if let Some(place_fields) = self.place_fields {
            query_parameters.push(("place.fields", place_fields.iter().join(",")));
        }
        if let Some(poll_fields) = self.poll_fields {
            query_parameters.push(("poll.fields", poll_fields.iter().join(",")));
        }
        if let Some(since_id) = self.since_id {
            query_parameters.push(("since_id", since_id));
        }
        if let Some(start_time) = self.start_time {
            query_parameters.push((
                "start_time",
                start_time.format("%Y-%m-%dT%H%M%SZ").to_string(),
            ));
        }
        if let Some(tweet_fields) = self.tweet_fields {
            query_parameters.push(("tweet.fields", tweet_fields.iter().join(",")));
        }
        if let Some(until_id) = self.until_id {
            query_parameters.push(("until_id", until_id));
        }
        if let Some(user_fields) = self.user_fields {
            query_parameters.push(("user.fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        client
            .get(URL.replace(":id", &self.id))
            .query(&query_parameters)
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}
