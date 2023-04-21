use crate::responses::{description::Description, urls::Urls};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Entities {
    pub urls: Option<Vec<Urls>>,
    pub description: Option<Description>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
