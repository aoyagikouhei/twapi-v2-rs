use crate::fields::{tweet_fields::TweetFields, user_fields::UserFields};
use crate::responses::{errors::Errors, includes::Includes, users::Users};
use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/users/by/username/:username";

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

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    username: String,
    expansions: Option<HashSet<Expansions>>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(bearer_code: &str, username: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            username: username.to_owned(),
            ..Default::default()
        }
    }

    pub fn all(bearer_code: &str, username: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            username: username.to_owned(),
            expansions: Some(Expansions::all()),
            tweet_fields: Some(TweetFields::all()),
            user_fields: Some(UserFields::all()),
        }
    }

    pub fn open(bearer_code: &str, username: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            username: username.to_owned(),
            expansions: Some(Expansions::all()),
            tweet_fields: Some(TweetFields::open()),
            user_fields: Some(UserFields::all()),
        }
    }

    pub fn expansions(mut self, value: HashSet<Expansions>) -> Self {
        self.expansions = Some(value);
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
        if let Some(expansions) = self.expansions {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(tweet_fields) = self.tweet_fields {
            query_parameters.push(("tweet.fields", tweet_fields.iter().join(",")));
        }
        if let Some(user_fields) = self.user_fields {
            query_parameters.push(("user.fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        client
            .get(URL.replace(":username", &self.username))
            .query(&query_parameters)
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Users>,
    pub errors: Option<Vec<Errors>>,
    pub includes: Option<Includes>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
