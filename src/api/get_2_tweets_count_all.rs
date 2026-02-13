use crate::responses::{counts::Counts, errors::Errors, meta_count::MetaCount};
use crate::{
    api::{Authentication, TwapiOptions, execute_twitter, make_url},
    error::Error,
    headers::Headers,
};
use chrono::prelude::*;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/2/tweets/counts/all";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
#[derive(Default)]
pub enum Granularity {
    #[serde(rename = "minute")]
    #[default]
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


#[derive(Debug, Clone, Default)]
pub struct Api {
    query: String,
    end_time: Option<DateTime<Utc>>,
    granularity: Option<Granularity>,
    since_id: Option<String>,
    start_time: Option<DateTime<Utc>>,
    until_id: Option<String>,
    twapi_options: Option<TwapiOptions>,
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

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("query", self.query.to_string()));
        if let Some(end_time) = self.end_time.as_ref() {
            query_parameters.push((
                "end_time",
                end_time.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            ));
        }
        if let Some(granularity) = self.granularity.as_ref() {
            query_parameters.push(("granularity", granularity.to_string()));
        }
        if let Some(since_id) = self.since_id.as_ref() {
            query_parameters.push(("since_id", since_id.to_string()));
        }
        if let Some(start_time) = self.start_time.as_ref() {
            query_parameters.push((
                "start_time",
                start_time.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            ));
        }
        if let Some(until_id) = self.until_id.as_ref() {
            query_parameters.push(("until_id", until_id.to_string()));
        }
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, URL);
        let builder = client.get(&url).query(&query_parameters);
        authentication.execute(
            builder,
            "GET",
            &url,
            &query_parameters
                .iter()
                .map(|it| (it.0, it.1.as_str()))
                .collect::<Vec<_>>(),
        )
    }

    pub async fn execute(
        &self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Headers), Error> {
        execute_twitter(|| self.build(authentication), &self.twapi_options).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Counts>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Errors>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<MetaCount>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .data
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .errors
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .meta
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
