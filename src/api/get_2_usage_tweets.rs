use crate::fields::usage_fields::UsageFields;
use crate::responses::{
    daily_client_app_usage::DailyClientAppUsage, daily_project_usage::DailyProjectUsage,
    errors::Errors, includes::Includes, meta::Meta,
};
use crate::{
    api::{apply_options, execute_twitter, make_url, Authentication, TwapiOptions},
    error::Error,
    headers::Headers,
};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "/2/usage/tweets";

#[derive(Debug, Clone, Default)]
pub struct Api {
    usage_fields: Option<HashSet<UsageFields>>,
    days: Option<usize>,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn all() -> Self {
        Self {
            usage_fields: Some(UsageFields::all()),
            days: Some(90),
            ..Default::default()
        }
    }

    pub fn usage_fields(mut self, value: HashSet<UsageFields>) -> Self {
        self.usage_fields = Some(value);
        self
    }

    pub fn days(mut self, value: usize) -> Self {
        self.days = Some(value);
        self
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(usage_fields) = self.usage_fields {
            query_parameters.push(("usage.fields", usage_fields.iter().join(",")));
        }
        if let Some(days) = self.days {
            query_parameters.push(("days", days.to_string()));
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Errors>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Includes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
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
                .unwrap_or(true)
            && self
                .errors
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .includes
                .as_ref()
                .map(|it| it.is_empty_extra())
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_reset_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_client_app_usage: Option<Vec<DailyClientAppUsage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_project_usage: Option<DailyProjectUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_usage: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .daily_client_app_usage
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .daily_project_usage
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
