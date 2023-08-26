use crate::responses::compliance::Compliance;
use crate::{
    api::{execute_twitter, Auth},
    error::Error,
    rate_limit::RateLimit,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/tweets/compliance/stream";

#[derive(Debug, Clone, Default)]
pub struct Api {
    partition: usize,
    backfill_minutes: Option<usize>,
}

impl Api {
    pub fn new(partition: usize) -> Self {
        Self {
            partition,
            ..Default::default()
        }
    }

    pub fn backfill_minutes(mut self, value: usize) -> Self {
        self.backfill_minutes = Some(value);
        self
    }

    pub fn build(self, auth: &impl Auth) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("partition", self.partition.to_string()));
        if let Some(backfill_minutes) = self.backfill_minutes {
            query_parameters.push(("backfill_minutes", backfill_minutes.to_string()));
        }
        let client = reqwest::Client::new();
        let builder = client.get(URL).query(&query_parameters);
        auth.auth(
            builder,
            "GET",
            URL,
            &query_parameters
                .iter()
                .map(|it| (it.0, it.1.as_str()))
                .collect(),
        )
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(auth)).await
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub delete: Option<Compliance>,
    pub withheld: Option<Compliance>,
    pub drop: Option<Compliance>,
    pub undrop: Option<Compliance>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .delete
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .withheld
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .drop
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .undrop
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
