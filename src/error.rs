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

    #[error("Invalid Parameter {0}")]
    InvalidParameter(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterError {
    pub status_code: u64,
    pub status: Option<u64>,
    pub detail: Option<String>,
    pub title: String,
    pub r#type: String,
    pub value: serde_json::Value,
}

impl TwitterError {
    pub fn new(status_code: reqwest::StatusCode, value: serde_json::Value) -> Self {
        let title = match value["title"].as_str() {
            Some(res) => res.to_owned(),
            None => match value["error"].as_str() {
                Some(res) => res.to_owned(),
                None => "no_title".to_owned(),
            },
        };
        let r#type = match value["type"].as_str() {
            Some(res) => res.to_owned(),
            None => match value["error"].as_str() {
                Some(res) => res.to_owned(),
                None => "no_type".to_owned(),
            },
        };
        let detail = match value["detail"].as_str() {
            Some(res) => Some(res.to_owned()),
            None => value["error_description"].as_str().map(|it| it.to_owned()),
        };
        Self {
            status_code: status_code.as_u16() as u64,
            status: value["status"].as_u64(),
            detail,
            title,
            r#type,
            value,
        }
    }
}

impl std::fmt::Display for TwitterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
