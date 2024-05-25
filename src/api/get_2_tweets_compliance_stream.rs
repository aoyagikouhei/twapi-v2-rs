use crate::responses::compliance::Compliance;
use crate::{
    api::{apply_options, execute_twitter, make_url, Authentication, TwapiOptions},
    error::Error,
    headers::Headers,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/2/tweets/compliance/stream";

#[derive(Debug, Clone, Default)]
pub struct Api {
    partition: usize,
    backfill_minutes: Option<usize>,
    twapi_options: Option<TwapiOptions>,
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

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("partition", self.partition.to_string()));
        if let Some(backfill_minutes) = self.backfill_minutes {
            query_parameters.push(("backfill_minutes", backfill_minutes.to_string()));
        }
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, URL);
        let builder = client.get(&url).query(&query_parameters);
        authentication.execute(
            apply_options(builder, &self.twapi_options),
            "GET",
            &url,
            &query_parameters
                .iter()
                .map(|it| (it.0, it.1.as_str()))
                .collect::<Vec<_>>(),
        )
    }

    pub async fn execute(
        self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Headers), Error> {
        execute_twitter(self.build(authentication)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Compliance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withheld: Option<Compliance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<Compliance>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
