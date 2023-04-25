use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Hashtags {
    pub start: Option<String>,
    pub end: Option<String>,
    pub hashtag: Option<String>,
    pub tag: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
