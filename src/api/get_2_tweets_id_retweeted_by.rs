use super::{execute_twitter, TwitterResult};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/tweets/:id/retweeted_by";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    PinnedTweetId,
}

impl Expansions {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::PinnedTweetId);
        result
    }
}

impl std::fmt::Display for Expansions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PinnedTweetId => write!(f, "pinned_tweet_id"),
        }
    }
}

impl Default for Expansions {
    fn default() -> Self {
        Self::PinnedTweetId
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
    max_results: Option<usize>,
    expansions: Option<HashSet<Expansions>>,
    pagination_token: Option<String>,
    tweet_fields: Option<HashSet<TweetFields>>,
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

    pub fn max_results(mut self, value: usize) -> Self {
        self.max_results = Some(value);
        self
    }

    pub fn expansions(mut self, value: HashSet<Expansions>) -> Self {
        self.expansions = Some(value);
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
        if let Some(max_results) = self.max_results {
            query_parameters.push(("max_results", max_results.to_string()));
        }
        if let Some(expansions) = self.expansions {
            query_parameters.push(("expansions", expansions.iter().join(",")));
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
            .get(URL.replace(":id", &self.id))
            .query(&query_parameters)
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}
