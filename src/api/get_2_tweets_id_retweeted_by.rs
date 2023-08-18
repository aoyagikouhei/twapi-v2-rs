use itertools::Itertools;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::fields::{tweet_fields::TweetFields, user_fields::UserFields};
use crate::responses::{users::Users, errors::Errors, includes::Includes, meta::Meta};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

const URL: &str = "https://api.twitter.com/2/tweets/:id/retweeted_by";



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
    fn default() -> Self { Self::PinnedTweetId }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    id: String,
    max_results: Option<usize>,
    expansions: Option<HashSet<Expansions>>,
    pagination_token: Option<String>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            ..Default::default()
        }
    }
    
    pub fn all(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            expansions: Some(Expansions::all()),
            tweet_fields: Some(TweetFields::organic()),
            user_fields: Some(UserFields::all()),
            max_results: Some(100),
            ..Default::default()
        }
    }
    
    pub fn open(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            expansions: Some(Expansions::all()),
            tweet_fields: Some(TweetFields::open()),
            user_fields: Some(UserFields::all()),
            max_results: Some(100),
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
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(), auth).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<Users>>, 
    pub errors: Option<Vec<Errors>>, 
    pub includes: Option<Includes>, 
    pub meta: Option<Meta>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.errors.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.includes.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.meta.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}
