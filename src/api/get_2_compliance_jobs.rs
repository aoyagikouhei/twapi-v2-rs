use crate::responses::{jobs::Jobs, meta::Meta};
use crate::{
    api::{Authentication, TwapiOptions, execute_twitter, make_url},
    error::Error,
    headers::Headers,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/2/compliance/jobs";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
#[derive(Default)]
pub enum Type {
    #[serde(rename = "tweets")]
    #[default]
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


#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
#[derive(Default)]
pub enum Status {
    #[serde(rename = "created")]
    #[default]
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


#[derive(Debug, Clone, Default)]
pub struct Api {
    r#type: Type,
    status: Option<Status>,
    twapi_options: Option<TwapiOptions>,
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

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("type", self.r#type.to_string()));
        if let Some(status) = self.status.as_ref() {
            query_parameters.push(("status", status.to_string()));
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
    pub data: Option<Vec<Jobs>>,
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
