use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use crate::responses::{counts::Counts, errors::Errors, meta_count::MetaCount};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

const URL: &str = "https://api.twitter.com/2/tweets/counts/all";



#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Granularity {
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
}

impl std::fmt::Display for Granularity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Minute => write!(f, "minute"),
            Self::Hour => write!(f, "hour"),
            Self::Day => write!(f, "day"),
        }
    }
}

impl Default for Granularity {
    fn default() -> Self { Self::Minute }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    query: String,
    end_time: Option<DateTime<Utc>>,
    granularity: Option<Granularity>,
    since_id: Option<String>,
    start_time: Option<DateTime<Utc>>,
    until_id: Option<String>,
}

impl Api {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_owned(),
            ..Default::default()
        }
    }
    
    pub fn end_time(mut self, value: DateTime<Utc>) -> Self {
        self.end_time = Some(value);
        self
    }
    
    pub fn granularity(mut self, value: Granularity) -> Self {
        self.granularity = Some(value);
        self
    }
    
    pub fn since_id(mut self, value: &str) -> Self {
        self.since_id = Some(value.to_owned());
        self
    }
    
    pub fn start_time(mut self, value: DateTime<Utc>) -> Self {
        self.start_time = Some(value);
        self
    }
    
    pub fn until_id(mut self, value: &str) -> Self {
        self.until_id = Some(value.to_owned());
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("query", self.query));
        if let Some(end_time) = self.end_time {
            query_parameters.push(("end_time", end_time.format("%Y-%m-%dT%H%M%SZ").to_string()));
        }
        if let Some(granularity) = self.granularity {
            query_parameters.push(("granularity", granularity.to_string()));
        }
        if let Some(since_id) = self.since_id {
            query_parameters.push(("since_id", since_id));
        }
        if let Some(start_time) = self.start_time {
            query_parameters.push(("start_time", start_time.format("%Y-%m-%dT%H%M%SZ").to_string()));
        }
        if let Some(until_id) = self.until_id {
            query_parameters.push(("until_id", until_id));
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
    pub data: Option<Vec<Counts>>, 
    pub errors: Option<Vec<Errors>>, 
    pub meta: Option<MetaCount>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.errors.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.meta.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}
