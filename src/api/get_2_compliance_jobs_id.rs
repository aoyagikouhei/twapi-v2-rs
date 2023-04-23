use serde::{Serialize, Deserialize};
use crate::responses::{jobs::Jobs};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::execute_twitter};

const URL: &str = "https://api.twitter.com/2/compliance/jobs/:id";





#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    id: String,
}

impl Api {
    pub fn new(bearer_code: &str, id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            id: id.to_owned(),
        }
    }
    
    pub fn build(self) -> RequestBuilder {
        
        let client = reqwest::Client::new();
        client
            .get(URL.replace(":id", &self.id))
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Jobs>, 
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
