use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/users/:source_user_id/muting/:target_user_id";

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    source_user_id: String,
    target_user_id: String,
}

impl Api {
    pub fn new(bearer_code: &str, source_user_id: &str, target_user_id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            source_user_id: source_user_id.to_owned(),
            target_user_id: target_user_id.to_owned(),
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .delete(
                URL.replace(":source_user_id", &self.source_user_id)
                    .replace(":target_user_id", &self.target_user_id),
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
    pub muting: Option<bool>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
