use crate::fields::{tweet_fields::TweetFields, user_fields::UserFields};
use crate::responses::{errors::Errors, includes::Includes, users::Users};
use crate::{
    api::{execute_twitter, Authentication},
    error::Error,
    rate_limit::RateLimit,
};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/users";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    #[serde(rename = "pinned_tweet_id")]
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
    ids: String,
    expansions: Option<HashSet<Expansions>>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(ids: &str) -> Self {
        Self {
            ids: ids.to_owned(),
            ..Default::default()
        }
    }

    pub fn all(ids: &str) -> Self {
        Self {
            ids: ids.to_owned(),
            expansions: Some(Expansions::all()),
            tweet_fields: Some(TweetFields::organic()),
            user_fields: Some(UserFields::all()),
        }
    }

    pub fn open(ids: &str) -> Self {
        Self {
            ids: ids.to_owned(),
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

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("ids", self.ids));
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
        let builder = client.get(URL).query(&query_parameters);
        authentication.execute(
            builder,
            "GET",
            URL,
            &query_parameters
                .iter()
                .map(|it| (it.0, it.1.as_str()))
                .collect::<Vec<_>>(),
        )
    }

    pub async fn execute(
        self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(authentication)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<Users>>,
    pub errors: Option<Vec<Errors>>,
    pub includes: Option<Includes>,
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
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
