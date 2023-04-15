use std::collections::HashSet;

use reqwest::RequestBuilder;
use itertools::Itertools;

use crate::fields::{tweet_fields::TweetFields, user_fields::UserFields};

use super::{TwitterResult, execute_twitter};
const URL: &str = "https://api.twitter.com/2/tweets/:id/liking_users";

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expantions {
    PinnedTweetId
}

impl Expantions {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::PinnedTweetId);
        result
    }
}

impl std::fmt::Display for Expantions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expantions::PinnedTweetId => write!(f, "pinned_tweet_id"),
        }
    }
}

#[derive(Debug)]
pub struct Api {
    max_results: Option<usize>,
    pagination_token: Option<String>,
    expansions: Option<HashSet<Expantions>>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
    id: String,
}

impl Api {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            expansions: None,
            max_results: None,
            pagination_token: None,
            tweet_fields: None,
            user_fields: None
        }
    }

    pub fn max_results(mut self, value: usize) -> Self {
        self.max_results = Some(value);
        self
    }

    pub fn pagination_token(mut self, value: &str) -> Self {
        self.pagination_token = Some(value.to_owned());
        self
    }

    pub fn expansions(mut self, value: HashSet<Expantions>) -> Self {
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
        if let Some(max_results) = self.max_results {
            query_parameters.push(("max_results", max_results.to_string()));
        }
        if let Some(pagination_token) = self.pagination_token {
            query_parameters.push(("pagination_token", pagination_token.to_owned()));
        }
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
            .get(URL.replace(":id", &self.id))
            .query(&query_parameters)
            .bearer_auth(bearer_code)
    }

    pub async fn execute(self, bearer_code: &str) -> TwitterResult {
        execute_twitter(self.build(bearer_code)).await
    }
}
