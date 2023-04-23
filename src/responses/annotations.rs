use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Annotations {
    pub start: Option<String>,
    pub end: Option<String>,
    pub probability: Option<String>,
    pub r#type: Option<String>,
    pub normalized_text: Option<String>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
