use chrono::prelude::*;
use crate::fields::{media_fields::MediaFields, place_fields::PlaceFields, poll_fields::PollFields, tweet_fields::TweetFields, user_fields::UserFields, };
use itertools::Itertools;
use reqwest::RequestBuilder;
use std::collections::HashSet;
use super::{TwitterResult, execute_twitter};

const URL: &str = "https://api.twitter.com/2/tweets/search/recent";

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum SortOrder {
    Recency,
    Relevancy,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Recency => write!(f, "recency"),
            Self::Relevancy => write!(f, "relevancy"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Api {
    query: String,
    end_time: Option<DateTime<Utc>>,
    expansions: Option<HashSet<Expansions>>,
    max_results: Option<usize>,
    media_fields: Option<HashSet<MediaFields>>,
    next_token: Option<String>,
    place_fields: Option<HashSet<PlaceFields>>,
    poll_fields: Option<HashSet<PollFields>>,
    since_id: Option<String>,
    sort_order: Option<SortOrder>,
    start_time: Option<DateTime<Utc>>,
    tweet_fields: Option<HashSet<TweetFields>>,
    until_id: Option<String>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_owned(),
            ..Default::default()
        }
    }
    
    pub fn end_time(mut self, value: DateTime<Utc>) -> Self {
        self.end_time = Some(value);
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
    
    pub fn next_token(mut self, value: &str) -> Self {
        self.next_token = Some(value.to_owned());
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
    
    pub fn sort_order(mut self, value: SortOrder) -> Self {
        self.sort_order = Some(value);
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
    

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("query", self.query));
        if let Some(end_time) = self.end_time {
            query_parameters.push(("end_time", end_time.format("%Y-%m-%dT%H%M%SZ").to_string()));
        }
        if let Some(expansions) = self.expansions {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(max_results) = self.max_results {
            query_parameters.push(("max_results", max_results.to_string()));
        }
        if let Some(media_fields) = self.media_fields {
            query_parameters.push(("media_fields", media_fields.iter().join(",")));
        }
        if let Some(next_token) = self.next_token {
            query_parameters.push(("next_token", next_token));
        }
        if let Some(place_fields) = self.place_fields {
            query_parameters.push(("place_fields", place_fields.iter().join(",")));
        }
        if let Some(poll_fields) = self.poll_fields {
            query_parameters.push(("poll_fields", poll_fields.iter().join(",")));
        }
        if let Some(since_id) = self.since_id {
            query_parameters.push(("since_id", since_id));
        }
        if let Some(sort_order) = self.sort_order {
            query_parameters.push(("sort_order", sort_order.to_string()));
        }
        if let Some(start_time) = self.start_time {
            query_parameters.push(("start_time", start_time.format("%Y-%m-%dT%H%M%SZ").to_string()));
        }
        if let Some(tweet_fields) = self.tweet_fields {
            query_parameters.push(("tweet_fields", tweet_fields.iter().join(",")));
        }
        if let Some(until_id) = self.until_id {
            query_parameters.push(("until_id", until_id));
        }
        if let Some(user_fields) = self.user_fields {
            query_parameters.push(("user_fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        client
            .get(URL)
            .query(&query_parameters)
            .bearer_auth(bearer_code)
    }

    pub async fn execute(self, bearer_code: &str) -> TwitterResult {
        execute_twitter(self.build(bearer_code)).await
    }
}