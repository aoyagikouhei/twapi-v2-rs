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

impl Compliance {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .tweet
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .user
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Compliance {:?}", self.extra);
        }
        res
    }
}
