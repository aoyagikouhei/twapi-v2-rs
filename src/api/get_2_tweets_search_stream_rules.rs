use crate::responses::{errors::Errors, streams::Streams};
use crate::{
    api::{apply_options, execute_twitter, make_url, Authentication, TwapiOptions},
    error::Error,
    headers::Headers,
};
use chrono::prelude::*;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/2/tweets/search/stream/rules";

#[derive(Debug, Clone, Default)]
pub struct Api {
    ids: Option<String>,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn ids(mut self, value: &str) -> Self {
        self.ids = Some(value.to_owned());
        self
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(ids) = self.ids {
            query_parameters.push(("ids", ids));
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
    pub data: Option<Vec<Streams>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Errors>>,
    pub meta: Meta,
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
            && self.meta.is_empty_extra();
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Meta {
    pub result_count: i64,
    pub sent: DateTime<Utc>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Meta {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Meta {:?}", self.extra);
        }
        res
    }
}
