use super::{execute_twitter, TwitterResult};
use reqwest::RequestBuilder;

const URL: &str = "https://api.twitter.com/2/oauth2/token";

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum GrantType {
    RefreshToken,
}

impl std::fmt::Display for GrantType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RefreshToken => write!(f, "refresh_token"),
        }
    }
}

impl Default for GrantType {
    fn default() -> Self {
        Self::RefreshToken
    }
}
#[derive(Debug, Clone, Default)]
pub struct Api {
    api_key_code: String,
    api_secret_code: String,
    client_id: String,
    grant_type: GrantType,
    refresh_token: String,
}

impl Api {
    pub fn new(
        api_key_code: &str,
        api_secret_code: &str,
        client_id: &str,
        grant_type: GrantType,
        refresh_token: &str,
    ) -> Self {
        Self {
            api_key_code: api_key_code.to_owned(),
            api_secret_code: api_secret_code.to_owned(),
            client_id: client_id.to_owned(),
            grant_type,
            refresh_token: refresh_token.to_owned(),
        }
    }

    pub fn build(self) -> RequestBuilder {
        let form_paramters = vec![
            ("client_id", self.client_id),
            ("grant_type", self.grant_type.to_string()),
            ("refresh_token", self.refresh_token),
        ];

        let client = reqwest::Client::new();
        client
            .post(URL)
            .form(&form_paramters)
            .basic_auth(self.api_key_code, Some(self.api_secret_code))
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}
