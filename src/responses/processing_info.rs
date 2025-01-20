use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProcessingInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_after_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
