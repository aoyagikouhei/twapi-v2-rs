use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Jobs {
    pub id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub upload_url: Option<String>,
    pub upload_expires_at: Option<DateTime<Utc>>,
    pub download_url: Option<String>,
    pub download_expires_at: Option<DateTime<Utc>>,
    pub url: Option<String>,
    pub status: Option<Status>,
    pub error: Option<String>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    InProgress,
    Failed,
    Complete,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InProgress => write!(f, "in_progress"),
            Self::Failed => write!(f, "failed"),
            Self::Complete => write!(f, "complete"),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::InProgress
    }
}
