use crate::fields::{
    media_fields::MediaFields, place_fields::PlaceFields, poll_fields::PollFields,
    tweet_fields::TweetFields, user_fields::UserFields,
};
use crate::responses::{errors::Errors, includes::Includes, tweets::Tweets};
use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use chrono::prelude::*;
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/users/:id/tweets";

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
    pub fn all(bearer_code: &str, id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            id: id.to_owned(),
            exclude: Some(Exclude::all()),
            expansions: Some(Expansions::all()),
            media_fields: Some(MediaFields::all()),
            place_fields: Some(PlaceFields::all()),
            poll_fields: Some(PollFields::all()),
            tweet_fields: Some(TweetFields::all()),
            user_fields: Some(UserFields::all()),
            ..Default::default()
        }
    }

    pub fn open(bearer_code: &str, id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            id: id.to_owned(),
            exclude: Some(Exclude::all()),
            expansions: Some(Expansions::all()),
            media_fields: Some(MediaFields::open()),
            place_fields: Some(PlaceFields::all()),
            poll_fields: Some(PollFields::all()),
            tweet_fields: Some(TweetFields::open()),
            user_fields: Some(UserFields::open()),
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

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<Tweets>>,
    pub errors: Option<Vec<Errors>>,
    pub includes: Option<Includes>,
    pub meta: Meta,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Meta {
    pub count: i64,
    pub newest_id: String,
    pub oldest_id: String,
    pub next_token: Option<String>,
    pub previous_token: Option<String>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
