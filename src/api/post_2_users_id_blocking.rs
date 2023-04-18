use serde::{Serialize, Deserialize};
use reqwest::RequestBuilder;
use super::{TwitterResult, execute_twitter};

const URL: &str = "https://api.twitter.com/2/users/:id/blocking";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    target_user_id: String,
}



#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    id: String,
    body: Body,
}

impl Api {
    pub fn new(bearer_code: &str, id: &str, body: Body) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            id: id.to_owned(),
            body,
        }
    }
    
    pub fn build(self) -> RequestBuilder {
        
        let client = reqwest::Client::new();
        client
            .post(URL.replace(":id", &self.id))
            .bearer_auth(self.bearer_code)
            .json(&serde_json::to_value(&self.body).unwrap())
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}