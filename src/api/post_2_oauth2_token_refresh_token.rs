use serde::{Serialize, Deserialize};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

const URL: &str = "https://api.twitter.com/2/oauth2/token";





#[derive(Debug, Clone, Default)]
pub struct Api {
    refresh_token: String,
}

impl Api {
    pub fn new(refresh_token: &str) -> Self {
        Self {
            refresh_token: refresh_token.to_owned(),
        }
    }
    
    pub fn build(self) -> RequestBuilder {
        
        let form_parameters = vec![
            ("client_id", self.api_key_code.clone()),
            ("grant_type", "refresh_token".to_owned()),
            ("refresh_token", self.refresh_token),
        ];

        let client = reqwest::Client::new();
        client
            .post(URL)
            .form(&form_parameters)
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(), auth).await
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
