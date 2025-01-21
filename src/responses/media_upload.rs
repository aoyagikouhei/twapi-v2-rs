use crate::responses::processing_info::ProcessingInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MediaUpload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_info: Option<ProcessingInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl MediaUpload {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .processing_info
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("MediaUpload {:?}", self.extra);
        }
        res
    }
}
