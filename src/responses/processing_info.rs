use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ProcessingInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_after_secs: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ProcessingInfo {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("ProcessingInfo {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum State {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Succeeded => write!(f, "succeeded"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Pending => write!(f, "pending"),
            Self::Failed => write!(f, "failed"),
        }
    }
}

impl Default for State {
    fn default() -> Self { Self::Succeeded }
}
