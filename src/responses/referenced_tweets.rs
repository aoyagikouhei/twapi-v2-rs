use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReferencedTweets {
    pub id: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
