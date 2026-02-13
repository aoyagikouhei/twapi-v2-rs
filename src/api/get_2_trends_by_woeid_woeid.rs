use crate::fields::trend_fields::TrendFields;
use crate::responses::trends::Trends;
use crate::{
    api::{Authentication, TwapiOptions, execute_twitter, make_url},
    error::Error,
    headers::Headers,
};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "/2/trends/by/woeid/:woeid";

#[derive(Debug, Clone, Default)]
pub struct Api {
    woeid: String,
    max_trends: Option<usize>,
    trend_fields: Option<HashSet<TrendFields>>,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(woeid: &str) -> Self {
        Self {
            woeid: woeid.to_owned(),
            ..Default::default()
        }
    }

    pub fn all(woeid: &str) -> Self {
        Self {
            woeid: woeid.to_owned(),
            trend_fields: Some(TrendFields::all()),
            max_trends: Some(50),
            ..Default::default()
        }
    }

    pub fn max_trends(mut self, value: usize) -> Self {
        self.max_trends = Some(value);
        self
    }

    pub fn trend_fields(mut self, value: HashSet<TrendFields>) -> Self {
        self.trend_fields = Some(value);
        self
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(max_trends) = self.max_trends.as_ref() {
            query_parameters.push(("max_trends", max_trends.to_string()));
        }
        if let Some(trend_fields) = self.trend_fields.as_ref() {
            query_parameters.push(("trend.fields", trend_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, &URL.replace(":woeid", &self.woeid));
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
    pub data: Option<Vec<Trends>>,
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
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
