use crate::responses::{urls::Urls, hashtags::Hashtags, mentions::Mentions};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Description {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<Urls>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtags: Option<Vec<Hashtags>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Mentions>>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Description {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.urls.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.hashtags.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.mentions.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true);
        if !res {
          println!("Description {:?}", self.extra);
        }
        res
    }
}
