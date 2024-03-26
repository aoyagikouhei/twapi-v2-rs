use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Image {
    pub image_type: String,
    pub w: u64,
    pub h: u64,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Video {
    pub video_type: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProcessingInfo {
    pub state: String,
    pub check_after_secs: Option<u64>,
    pub progress_percent: Option<u64>,
    pub error: Option<Error>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Error {
    pub code: String,
    pub name: String,
    pub message: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub media_id: u64,
    pub media_id_string: String,
    pub expires_after_secs: u64,
    pub media_key: Option<String>,
    pub size: Option<u64>,
    pub image: Option<Image>,
    pub video: Option<Video>,
    pub processing_info: Option<ProcessingInfo>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
