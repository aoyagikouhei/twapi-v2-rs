use crate::responses::{compliance_tweet::ComplianceTweet, compliance_user::ComplianceUser};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Compliance {
    pub tweet: Option<ComplianceTweet>,
    pub user: Option<ComplianceUser>,
    pub event_at: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
