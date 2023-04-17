use super::{execute_twitter, TwitterResult};
use reqwest::RequestBuilder;

const URL: &str = "https://api.twitter.com/2/oauth2/token";

#[derive(Debug, Default)]
pub struct Api {
    client_id: String,
    grant_type: String,
    refresh_token: String,
}

impl Api {
    pub fn new(client_id: &str, grant_type: &str, refresh_token: &str) -> Self {
        Self {
            client_id: client_id.to_owned(),
            grant_type: grant_type.to_owned(),
            refresh_token: refresh_token.to_owned(),
        }
    }

    pub fn build(self, api_key_code: &str, api_secret_code: &str) -> RequestBuilder {
        let form_paramters = vec![
            ("client_id", self.client_id),
            ("grant_type", self.grant_type),
            ("refresh_token", self.refresh_token),
        ];

        let client = reqwest::Client::new();
        client
            .post(URL)
            .form(&form_paramters)
            .basic_auth(api_key_code, Some(api_secret_code))
    }

    pub async fn execute(self, api_key_code: &str, api_secret_code: &str) -> TwitterResult {
        execute_twitter(self.build(api_key_code, api_secret_code)).await
    }
}
