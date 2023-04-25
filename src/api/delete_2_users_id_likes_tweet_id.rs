use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/users/:id/likes/:tweet_id";

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    id: String,
    tweet_id: String,
}

impl Api {
    pub fn new(bearer_code: &str, id: &str, tweet_id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            id: id.to_owned(),
            tweet_id: tweet_id.to_owned(),
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .delete(
                URL.replace(":id", &self.id)
                    .replace(":tweet_id", &self.tweet_id),
            )
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Data>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub liked: Option<bool>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
