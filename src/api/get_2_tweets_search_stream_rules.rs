use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use crate::responses::{streams::Streams, errors::Errors};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

const URL: &str = "https://api.twitter.com/2/tweets/search/stream/rules";





#[derive(Debug, Clone, Default)]
pub struct Api {
    ids: Option<String>,
}

impl Api {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    
    pub fn ids(mut self, value: &str) -> Self {
        self.ids = Some(value.to_owned());
        self
    }

    pub fn build(self, auth: &impl Auth) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(ids) = self.ids {
            query_parameters.push(("ids", ids));
        }
        let client = reqwest::Client::new();
        let builder = client
            .get(URL)
            .query(&query_parameters)
        ;
        auth.auth(builder, "get", URL, &query_parameters.iter().map(|it| (it.0, it.1.as_str())).collect())
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(auth)).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<Streams>>, 
    pub errors: Option<Vec<Errors>>, 
    pub meta: Meta, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.errors.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.meta.is_empty_extra();
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Meta {
    pub result_count: i64, 
    pub sent: DateTime<Utc>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Meta {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Meta {:?}", self.extra);
        }
        res
    }
}
