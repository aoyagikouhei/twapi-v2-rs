use crate::responses::jobs::Jobs;
use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/compliance/jobs";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Type {
    #[serde(rename = "tweets")]
    Tweets,
    #[serde(rename = "users")]
    Users,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tweets => write!(f, "tweets"),
            Self::Users => write!(f, "users"),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Tweets
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub r#type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumable: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    body: Body,
}

impl Api {
    pub fn new(bearer_code: &str, body: Body) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            body,
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(URL)
            .bearer_auth(self.bearer_code)
            .json(&self.body)
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
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
        let res = self.extra.is_empty()
            && self
                .data
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
