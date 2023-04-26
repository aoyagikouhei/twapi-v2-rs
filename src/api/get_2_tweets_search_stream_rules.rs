use crate::responses::{errors::Errors, streams::Streams};
use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/tweets/search/stream/rules";

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    ids: Option<String>,
}

impl Api {
    pub fn new(bearer_code: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            ..Default::default()
        }
    }

    pub fn ids(mut self, value: &str) -> Self {
        self.ids = Some(value.to_owned());
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(ids) = self.ids {
            query_parameters.push(("ids", ids));
        }
        let client = reqwest::Client::new();
        client
            .get(URL)
            .query(&query_parameters)
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
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
            && self.meta.is_empty_extra();
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Meta {
    pub sent: i64,
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
