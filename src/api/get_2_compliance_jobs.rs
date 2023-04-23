use itertools::Itertools;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::responses::{jobs::Jobs};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::execute_twitter};

const URL: &str = "https://api.twitter.com/2/compliance/jobs";



#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Type {
    Tweets,
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
    Created,
    InProgress,
    Failed,
    Complete,
}

impl Status {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Created);
        result.insert(Self::InProgress);
        result.insert(Self::Failed);
        result.insert(Self::Complete);
        result
    }
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
    bearer_code: String,
    r#type: Type,
    status: Option<HashSet<Status>>,
}

impl Api {
    pub fn new(bearer_code: &str, r#type: Type) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            r#type,
            ..Default::default()
        }
    }
    
    pub fn status(mut self, value: HashSet<Status>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("type", self.r#type.to_string()));
        if let Some(status) = self.status {
            query_parameters.push(("status", status.iter().join(",")));
        }
        let client = reqwest::Client::new();
        client
            .get(URL)
            .query(&query_parameters)
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<Jobs>>, 
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
