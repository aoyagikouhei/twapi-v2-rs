use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::rate_limit::RateLimit;

#[derive(Error, Debug)]
pub enum Error {
    #[error("reqwest {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("serde json {0}")]
    Json(#[from] serde_json::Error),

    #[error("Twitter {0}")]
    Twitter(TwitterError, Option<RateLimit>),

    #[error("Fields {2}")]
    Fields(u64, Vec<FieldError>, serde_json::Value, Option<RateLimit>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldError {
    pub detail: String,
    pub field: String,
    pub parameter: String,
    pub resource_id: String,
    pub resource_type: String,
    pub section: String,
    pub title: String,
    pub r#type: String,
    pub value: String,
}

impl FieldError {
    pub fn new(source: &serde_json::Value) -> Self {
        Self {
            detail: source["detail"].as_str().unwrap_or_default().to_owned(),
            field: source["field"].as_str().unwrap_or_default().to_owned(),
            parameter: source["parameter"].as_str().unwrap_or_default().to_owned(),
            resource_id: source["resource_id"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
            resource_type: source["resource_type"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
            section: source["section"].as_str().unwrap_or_default().to_owned(),
            title: source["title"].as_str().unwrap_or_default().to_owned(),
            r#type: source["type"].as_str().unwrap_or_default().to_owned(),
            value: source["value"].as_str().unwrap_or_default().to_owned(),
        }
    }

    pub fn field_error(
        status_code: reqwest::StatusCode,
        source: serde_json::Value,
        rate_limit: Option<RateLimit>,
    ) -> Error {
        let errors: Vec<Self> = match source["errors"].as_array() {
            Some(array) => array.iter().map(Self::new).collect(),
            None => vec![],
        };
        Error::Fields(status_code.as_u16() as u64, errors, source, rate_limit)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterError {
    pub status_code: u64,
    pub status: Option<u64>,
    pub detail: Option<String>,
    pub title: String,
    pub r#type: String,
    pub source: serde_json::Value,
}

impl TwitterError {
    pub fn new(status_code: reqwest::StatusCode, source: serde_json::Value) -> Self {
        let title = match source["title"].as_str() {
            Some(res) => res.to_owned(),
            None => match source["error"].as_str() {
                Some(res) => res.to_owned(),
                None => "no_title".to_owned(),
            },
        };
        let r#type = match source["type"].as_str() {
            Some(res) => res.to_owned(),
            None => match source["error"].as_str() {
                Some(res) => res.to_owned(),
                None => "no_type".to_owned(),
            },
        };
        let detail = match source["detail"].as_str() {
            Some(res) => Some(res.to_owned()),
            None => source["error_description"].as_str().map(|it| it.to_owned()),
        };
        Self {
            status_code: status_code.as_u16() as u64,
            status: source["status"].as_u64(),
            detail,
            title,
            r#type,
            source,
        }
    }
}

impl std::fmt::Display for TwitterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
