use crate::responses::{
    annotations::Annotations, cashtags::Cashtags, description::Description, hashtags::Hashtags,
    mentions::Mentions, urls::Urls,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Entities {
    pub annotations: Option<Vec<Annotations>>,
    pub cashtags: Option<Vec<Cashtags>>,
    pub hashtags: Option<Vec<Hashtags>>,
    pub mentions: Option<Vec<Mentions>>,
    pub urls: Option<Vec<Urls>>,
    pub description: Option<Description>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
