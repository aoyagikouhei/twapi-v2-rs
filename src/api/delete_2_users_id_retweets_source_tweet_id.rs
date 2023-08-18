use serde::{Serialize, Deserialize};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

const URL: &str = "https://api.twitter.com/2/users/:id/retweets/:source_tweet_id";





#[derive(Debug, Clone, Default)]
pub struct Api {
    id: String,
    source_tweet_id: String,
}

impl Api {
    pub fn new(id: &str, source_tweet_id: &str) -> Self {
        Self {
            id: id.to_owned(),
            source_tweet_id: source_tweet_id.to_owned(),
        }
    }
    
    pub fn build(self) -> RequestBuilder {
        
        let client = reqwest::Client::new();
        client
            .delete(URL.replace(":id", &self.id).replace(":source_tweet_id", &self.source_tweet_id))
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(), auth).await
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
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub retweeted: Option<bool>, 
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
