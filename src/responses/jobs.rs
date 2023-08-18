use serde::{Serialize, Deserialize};
use chrono::prelude::*;

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
    pub resumable: Option<bool>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Jobs {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Jobs {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "complete")]
    Complete,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Failed => write!(f, "failed"),
            Self::Complete => write!(f, "complete"),
        }
    }
}

impl Default for Status {
    fn default() -> Self { Self::Created }
}
