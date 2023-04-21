use crate::responses::urls::Urls;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Description {
    pub urls: Option<Vec<Urls>>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
