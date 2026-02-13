use itertools::Itertools;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::fields::{tweet_fields::TweetFields, user_fields::UserFields};
use crate::responses::{users::Users, errors::Errors, includes::Includes, meta::Meta};
use reqwest::RequestBuilder;
use crate::{error::Error, headers::Headers, api::{apply_options, execute_twitter, Authentication, make_url, TwapiOptions}};

const URL: &str = "/2/users/:id/followers";



#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    #[serde(rename = "affiliation.user_id")]
    AffiliationUserId,
    #[serde(rename = "most_recent_tweet_id")]
    MostRecentTweetId,
    #[serde(rename = "pinned_tweet_id")]
    PinnedTweetId,
}

impl Expansions {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::AffiliationUserId);
        result.insert(Self::MostRecentTweetId);
        result.insert(Self::PinnedTweetId);
        result
    }
}

impl std::fmt::Display for Expansions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AffiliationUserId => write!(f, "affiliation.user_id"),
            Self::MostRecentTweetId => write!(f, "most_recent_tweet_id"),
            Self::PinnedTweetId => write!(f, "pinned_tweet_id"),
        }
    }
}

impl Default for Expansions {
    fn default() -> Self { Self::AffiliationUserId }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    id: String,
    expansions: Option<HashSet<Expansions>>,
    max_results: Option<usize>,
    pagination_token: Option<String>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
    twapi_options: Option<TwapiOptions>,
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
            max_results: Some(1000),
            ..Default::default()
        }
    }
    
    pub fn open(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            expansions: Some(Expansions::all()),
            tweet_fields: Some(TweetFields::open()),
            user_fields: Some(UserFields::all()),
            max_results: Some(1000),
            ..Default::default()
        }
    }
    
    pub fn expansions(mut self, value: HashSet<Expansions>) -> Self {
        self.expansions = Some(value);
        self
    }
    
    pub fn max_results(mut self, value: usize) -> Self {
        self.max_results = Some(value);
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
    
    
    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(expansions) = self.expansions.as_ref() {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(max_results) = self.max_results.as_ref() {
            query_parameters.push(("max_results", max_results.to_string()));
        }
        if let Some(pagination_token) = self.pagination_token.as_ref() {
            query_parameters.push(("pagination_token", pagination_token.to_string()));
        }
        if let Some(tweet_fields) = self.tweet_fields.as_ref() {
            query_parameters.push(("tweet.fields", tweet_fields.iter().join(",")));
        }
        if let Some(user_fields) = self.user_fields.as_ref() {
            query_parameters.push(("user.fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, &URL.replace(":id", &self.id));
        let builder = client
            .get(&url)
            .query(&query_parameters)
        ;
        authentication.execute(apply_options(builder, &self.twapi_options), "GET", &url, &query_parameters.iter().map(|it| (it.0, it.1.as_str())).collect::<Vec<_>>())
    }

    pub async fn execute(&self, authentication: &impl Authentication) -> Result<(Response, Headers), Error> {
        execute_twitter(|| self.build(authentication)).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Users>>, 
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
