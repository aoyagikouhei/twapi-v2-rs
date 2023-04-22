use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Counts {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub tweet_count: Option<i64>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
