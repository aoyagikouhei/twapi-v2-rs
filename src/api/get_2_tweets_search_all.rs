use crate::fields::{
    media_fields::MediaFields, place_fields::PlaceFields, poll_fields::PollFields,
    tweet_fields::TweetFields, user_fields::UserFields,
};
use crate::responses::{errors::Errors, includes::Includes, meta::Meta, tweets::Tweets};
use crate::{
    api::{apply_options, execute_twitter, make_url, Authentication, TwapiOptions},
    error::Error,
    headers::Headers,
};
use chrono::prelude::*;
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "/2/tweets/search/all";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    #[serde(rename = "attachments.poll_ids")]
    AttachmentsPollIds,
    #[serde(rename = "attachments.media_keys")]
    AttachmentsMediaKeys,
    #[serde(rename = "author_id")]
    AuthorId,
    #[serde(rename = "edit_history_tweet_ids")]
    EditHistoryTweetIds,
    #[serde(rename = "entities.mentions.username")]
    EntitiesMentionsUsername,
    #[serde(rename = "geo.place_id")]
    GeoPlaceId,
    #[serde(rename = "in_reply_to_user_id")]
    InReplyToUserId,
    #[serde(rename = "referenced_tweets.id")]
    ReferencedTweetsId,
    #[serde(rename = "referenced_tweets.id.author_id")]
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
pub enum SortOrder {
    #[serde(rename = "recency")]
    Recency,
    #[serde(rename = "relevancy")]
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

impl Default for SortOrder {
    fn default() -> Self {
        Self::Recency
    }
}

#[derive(Debug, Clone, Default)]
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
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_owned(),
            ..Default::default()
        }
    }

    pub fn all(query: &str) -> Self {
        Self {
            query: query.to_owned(),
            expansions: Some(Expansions::all()),
            media_fields: Some(MediaFields::all()),
            place_fields: Some(PlaceFields::all()),
            poll_fields: Some(PollFields::all()),
            tweet_fields: Some(TweetFields::organic()),
            user_fields: Some(UserFields::all()),
            max_results: Some(500),
            ..Default::default()
        }
    }

    pub fn open(query: &str) -> Self {
        Self {
            query: query.to_owned(),
            expansions: Some(Expansions::all()),
            media_fields: Some(MediaFields::open()),
            place_fields: Some(PlaceFields::all()),
            poll_fields: Some(PollFields::all()),
            tweet_fields: Some(TweetFields::open()),
            user_fields: Some(UserFields::all()),
            max_results: Some(500),
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

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
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
            query_parameters.push(("media.fields", media_fields.iter().join(",")));
        }
        if let Some(next_token) = self.next_token {
            query_parameters.push(("next_token", next_token));
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
        if let Some(sort_order) = self.sort_order {
            query_parameters.push(("sort_order", sort_order.to_string()));
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
        let url = make_url(&self.twapi_options, URL);
        let builder = client.get(&url).query(&query_parameters);
        authentication.execute(
            apply_options(builder, &self.twapi_options),
            "GET",
            &url,
            &query_parameters
                .iter()
                .map(|it| (it.0, it.1.as_str()))
                .collect::<Vec<_>>(),
        )
    }

    pub async fn execute(
        self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Headers), Error> {
        execute_twitter(self.build(authentication)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Tweets>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Errors>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Includes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .data
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .errors
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .includes
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .meta
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
