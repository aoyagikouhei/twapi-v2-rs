use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EditControls {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable_until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edits_remaining: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
