use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Attachments {
    pub media_keys: Option<Vec<String>>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
