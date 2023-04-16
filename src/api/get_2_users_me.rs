use super::{execute_twitter, TwitterResult};
use crate::fields::{tweet_fields::TweetFields, user_fields::UserFields};
use itertools::Itertools;
use reqwest::RequestBuilder;
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/users/me";

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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
#[derive(Debug, Default)]
pub struct Api {
    expansions: Option<HashSet<Expansions>>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new() -> Self {
        Self {
            ..Default::default()
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

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(expansions) = self.expansions {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(tweet_fields) = self.tweet_fields {
            query_parameters.push(("tweet_fields", tweet_fields.iter().join(",")));
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
