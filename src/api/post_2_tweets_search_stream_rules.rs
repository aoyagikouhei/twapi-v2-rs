use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use crate::responses::{streams::Streams, errors::Errors, summary::Summary};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

const URL: &str = "https://api.twitter.com/2/tweets/search/stream/rules";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Add {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Delete {
    pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<Add>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Delete>,
}



#[derive(Debug, Clone, Default)]
pub struct Api {
    dry_run: Option<bool>,
    body: Body,
}

impl Api {
    pub fn new(body: Body) -> Self {
        Self {
            body,
            ..Default::default()
        }
    }
    
    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn build(self, auth: &impl Auth) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(dry_run) = self.dry_run {
            query_parameters.push(("dry_run", dry_run.to_string()));
        }
        let client = reqwest::Client::new();
        let builder = client
            .post(URL)
            .query(&query_parameters)
            .json(&self.body)
        ;
        auth.auth(builder, "post", URL, &query_parameters.iter().map(|it| (it.0, it.1.as_str())).collect())
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
    pub sent: DateTime<Utc>, 
    pub summary: Summary, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Meta {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.summary.is_empty_extra();
        if !res {
          println!("Meta {:?}", self.extra);
        }
        res
    }
}
