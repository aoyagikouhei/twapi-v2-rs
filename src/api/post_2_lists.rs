use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/lists";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    name: String,
    description: Option<String>,
    private: Option<String>,
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
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
