use crate::responses::{
    annotations::Annotations, cashtags::Cashtags, hashtags::Hashtags, mentions::Mentions,
    urls::Urls,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Entities {
    pub annotations: Option<Vec<Annotations>>,
    pub cashtags: Option<Vec<Cashtags>>,
    pub hashtags: Option<Vec<Hashtags>>,
    pub mentions: Option<Vec<Mentions>>,
    pub urls: Option<Vec<Urls>>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Entities {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .annotations
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .cashtags
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .hashtags
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .mentions
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .urls
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Entities {:?}", self.extra);
        }
        res
    }
}
