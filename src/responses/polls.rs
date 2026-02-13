use crate::responses::options::Options;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Polls {
    pub id: String,
    pub options: Vec<Options>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_datetime: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voting_status: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Polls {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() && self.options.iter().all(|it| it.is_empty_extra());
        if !res {
            println!("Polls {:?}", self.extra);
        }
        res
    }
}
