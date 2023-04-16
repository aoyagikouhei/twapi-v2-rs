use super::{execute_twitter, TwitterResult};
use reqwest::RequestBuilder;

const URL: &str = "https://api.twitter.com/2/tweets/search/stream/rules";

#[derive(Debug, Default)]
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

    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(ids) = self.ids {
            query_parameters.push(("ids", ids));
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
