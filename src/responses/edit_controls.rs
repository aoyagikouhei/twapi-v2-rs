use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EditControls {
    pub editable_until: Option<DateTime<Utc>>, 
    pub edits_remaining: Option<i64>, 
    pub is_edit_eligible: Option<bool>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl EditControls {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("EditControls {:?}", self.extra);
        }
        res
    }
}
