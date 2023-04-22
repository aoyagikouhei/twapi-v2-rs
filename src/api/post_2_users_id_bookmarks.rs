use super::{execute_twitter, TwitterResult};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/users/:id/bookmarks";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    tweet_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    id: String,
    body: Body,
}

impl Api {
    pub fn new(bearer_code: &str, id: &str, body: Body) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            id: id.to_owned(),
            body,
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(URL.replace(":id", &self.id))
            .bearer_auth(self.bearer_code)
            .json(&serde_json::to_value(&self.body).unwrap())
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Data>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub bookmarked: Option<bool>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
