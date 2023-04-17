use super::{execute_twitter, TwitterResult};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/tweets/search/stream/rules";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Add {
    value: String,
    tag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Delete {
    ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    add: Option<Add>,
    delete: Option<Delete>,
}

#[derive(Debug, Default)]
pub struct Api {
    dry_run: Option<bool>,
}

impl Api {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn build(self, body: &Body, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(dry_run) = self.dry_run {
            query_parameters.push(("dry_run", dry_run.to_string()));
        }
        let client = reqwest::Client::new();
        client
            .post(URL)
            .query(&query_parameters)
            .bearer_auth(bearer_code)
            .json(&serde_json::to_value(body).unwrap())
    }

    pub async fn execute(self, body: &Body, bearer_code: &str) -> TwitterResult {
        execute_twitter(self.build(body, bearer_code)).await
    }
}
