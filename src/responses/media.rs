use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Media {
    pub height: Option<i64>,
    pub media_key: Option<String>,
    pub r#type: Option<String>,
    pub url: Option<String>,
    pub width: Option<i64>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
