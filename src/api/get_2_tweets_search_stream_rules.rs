use super::{execute_twitter, TwitterResult};
use crate::responses::{errors::Errors, streams::Streams};
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

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<Streams>>,
    pub errors: Option<Vec<Errors>>,
    pub meta: Meta,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Meta {
    pub sent: i64,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
