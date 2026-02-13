use crate::responses::{annotations::Annotations, cashtags::Cashtags, hashtags::Hashtags, mentions::Mentions, urls::Urls};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Entities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotations>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtags: Option<Vec<Cashtags>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtags: Option<Vec<Hashtags>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Mentions>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<Urls>>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Entities {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.annotations.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.cashtags.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.hashtags.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.mentions.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.urls.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true);
        if !res {
          println!("Entities {:?}", self.extra);
        }
        res
    }
}
