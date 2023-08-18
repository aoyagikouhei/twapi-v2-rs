use serde::{Serialize, Deserialize};
use crate::responses::{jobs::Jobs, meta::Meta};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

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
    fn default() -> Self { Self::Tweets }
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Status {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "complete")]
    Complete,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Failed => write!(f, "failed"),
            Self::Complete => write!(f, "complete"),
        }
    }
}

impl Default for Status {
    fn default() -> Self { Self::Created }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    r#type: Type,
    status: Option<Status>,
}

impl Api {
    pub fn new(r#type: Type) -> Self {
        Self {
            r#type,
            ..Default::default()
        }
    }
    
    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("type", self.r#type.to_string()));
        if let Some(status) = self.status {
            query_parameters.push(("status", status.to_string()));
        }
        let client = reqwest::Client::new();
        client
            .get(URL)
            .query(&query_parameters)
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(), auth).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<Jobs>>, 
    pub meta: Option<Meta>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.meta.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}
