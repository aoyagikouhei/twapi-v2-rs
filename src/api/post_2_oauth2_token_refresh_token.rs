use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/oauth2/token";

#[derive(Debug, Clone, Default)]
pub struct Api {
    api_key_code: String,
    api_secret_code: String,
    refresh_token: String,
}

impl Api {
    pub fn new(api_key_code: &str, api_secret_code: &str, refresh_token: &str) -> Self {
        Self {
            api_key_code: api_key_code.to_owned(),
            api_secret_code: api_secret_code.to_owned(),
            refresh_token: refresh_token.to_owned(),
        }
    }

    pub fn build(self) -> RequestBuilder {
        let form_paramters = vec![
            ("client_id", self.api_key_code.clone()),
            ("grant_type", "refresh_token".to_owned()),
            ("refresh_token", self.refresh_token),
        ];

        let client = reqwest::Client::new();
        client
            .post(URL)
            .form(&form_paramters)
            .basic_auth(self.api_key_code, Some(self.api_secret_code))
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i64>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
