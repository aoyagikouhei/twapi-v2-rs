use crate::{
    api::{TwapiOptions, execute_twitter, make_url},
    error::Error,
    headers::Headers,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::apply_options;

const URL: &str = "/2/oauth2/token";

#[derive(Debug, Clone, Default)]
pub struct Api {
    api_key_code: String,
    api_secret_code: String,
    refresh_token: String,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(api_key_code: &str, api_secret_code: &str, refresh_token: &str) -> Self {
        Self {
            api_key_code: api_key_code.to_owned(),
            api_secret_code: api_secret_code.to_owned(),
            refresh_token: refresh_token.to_owned(),
            ..Default::default()
        }
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self) -> RequestBuilder {
        let form_parameters = vec![
            ("client_id", self.api_key_code.clone()),
            ("grant_type", "refresh_token".to_owned()),
            ("refresh_token", self.refresh_token.to_string()),
        ];

        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, URL);
        apply_options(client.post(url), &self.twapi_options)
            .form(&form_parameters)
            .basic_auth(&self.api_key_code, Some(&self.api_secret_code))
    }

    pub async fn execute(&self) -> Result<(Response, Headers), Error> {
        execute_twitter(|| self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
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
