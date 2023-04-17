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

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    dry_run: Option<bool>,
    body: Body,
}

impl Api {
    pub fn new(bearer_code: &str, body: Body) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            body,
            ..Default::default()
        }
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(dry_run) = self.dry_run {
            query_parameters.push(("dry_run", dry_run.to_string()));
        }
        let client = reqwest::Client::new();
        client
            .post(URL)
            .query(&query_parameters)
            .bearer_auth(self.bearer_code)
            .json(&serde_json::to_value(&self.body).unwrap())
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}
