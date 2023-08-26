use crate::{
    api::{execute_twitter, Authentication},
    error::Error,
    rate_limit::RateLimit,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/users/:id/likes";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub tweet_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    id: String,
    body: Body,
}

impl Api {
    pub fn new(id: &str, body: Body) -> Self {
        Self {
            id: id.to_owned(),
            body,
        }
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let builder = client.post(URL.replace(":id", &self.id)).json(&self.body);
        authentication.execute(builder, "POST", URL, &vec![])
    }

    pub async fn execute(
        self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(authentication)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Data>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .data
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub liked: Option<bool>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
