use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::rate_limit::RateLimit;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Other {0}")]
    Other(String),

    #[error("reqwest {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("serde json {0}")]
    Json(#[from] serde_json::Error),

    #[error("Twitter {0:?}")]
    Twitter(Vec<TwitterError>, Option<RateLimit>),

    #[error("StatusError {0:?}")]
    Status(StatusError, Option<RateLimit>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterError {
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

impl TwitterError {
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

    pub fn new_vec(source: serde_json::Value) -> Vec<Self> {
        match source["errors"].as_array() {
            Some(array) => array.iter().map(Self::new).collect(),
            None => vec![],
        }
    }
}

// Err(Other(401, Object {"detail": String("Unauthorized"), "status": Number(401), "title": String("Unauthorized"), "type": String("about:blank")}, None))

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusError {
    pub status: u64,
    pub detail: String,
    pub title: String,
    pub r#type: String,
}

impl StatusError {
    pub fn new(source: serde_json::Value, status_code: StatusCode) -> Self {
        Self {
            status: source["status"]
                .as_u64()
                .unwrap_or(status_code.as_u16() as u64),
            detail: source["detail"].as_str().unwrap_or_default().to_owned(),
            title: source["title"].as_str().unwrap_or_default().to_owned(),
            r#type: source["type"].as_str().unwrap_or_default().to_owned(),
        }
    }
}
