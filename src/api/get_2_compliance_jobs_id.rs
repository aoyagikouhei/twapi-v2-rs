use serde::{Serialize, Deserialize};
use crate::responses::{jobs::Jobs};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

const URL: &str = "https://api.twitter.com/2/compliance/jobs/:id";





#[derive(Debug, Clone, Default)]
pub struct Api {
    id: String,
}

impl Api {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
        }
    }
    
    pub fn build(self, auth: &impl Auth) -> RequestBuilder {
        
        let client = reqwest::Client::new();
        let builder = client
            .get(URL.replace(":id", &self.id))
        ;
        auth.auth(builder, "get", URL, &vec![])
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(auth)).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Jobs>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}
