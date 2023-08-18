use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Counts {
    pub start: Option<DateTime<Utc>>, 
    pub end: Option<DateTime<Utc>>, 
    pub tweet_count: Option<i64>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Counts {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Counts {:?}", self.extra);
        }
        res
    }
}
