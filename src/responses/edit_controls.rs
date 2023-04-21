use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EditControls {
    pub editable_until: Option<DateTime<Utc>>,
    pub edits_remaining: Option<String>,
    pub is_edit_eligible: Option<bool>,
}