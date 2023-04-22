use crate::responses::options::Options;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Polls {
    pub id: String,
    pub options: Vec<Options>,
    pub duration_minutes: Option<i64>,
    pub end_datetime: Option<DateTime<Utc>>,
    pub voting_status: Option<String>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
