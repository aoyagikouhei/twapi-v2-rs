use crate::responses::compliance::Compliance;
use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/users/compliance/stream";

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    partition: usize,
    backfill_minutes: Option<usize>,
}

impl Api {
    pub fn new(bearer_code: &str, partition: usize) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            partition,
            ..Default::default()
        }
    }

    pub fn backfill_minutes(mut self, value: usize) -> Self {
        self.backfill_minutes = Some(value);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("partition", self.partition.to_string()));
        if let Some(backfill_minutes) = self.backfill_minutes {
            query_parameters.push(("backfill_minutes", backfill_minutes.to_string()));
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
    pub data: Option<Data>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub user_delete: Option<Compliance>,
    pub user_undelete: Option<Compliance>,
    pub user_withheld: Option<Compliance>,
    pub user_protect: Option<Compliance>,
    pub user_unprotect: Option<Compliance>,
    pub user_suspend: Option<Compliance>,
    pub user_unsuspend: Option<Compliance>,
    pub scrub_geo: Option<Compliance>,
    pub user_profile_modification: Option<Compliance>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
