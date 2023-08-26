use crate::{
    api::{execute_twitter, Auth},
    error::Error,
    rate_limit::RateLimit,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/users/:source_user_id/muting/:target_user_id";

#[derive(Debug, Clone, Default)]
pub struct Api {
    source_user_id: String,
    target_user_id: String,
}

impl Api {
    pub fn new(source_user_id: &str, target_user_id: &str) -> Self {
        Self {
            source_user_id: source_user_id.to_owned(),
            target_user_id: target_user_id.to_owned(),
        }
    }

    pub fn build(self, auth: &impl Auth) -> RequestBuilder {
        let client = reqwest::Client::new();
        let builder = client.delete(
            URL.replace(":source_user_id", &self.source_user_id)
                .replace(":target_user_id", &self.target_user_id),
        );
        auth.auth(builder, "delete", URL, &vec![])
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(auth)).await
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
    pub muting: Option<bool>,
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
