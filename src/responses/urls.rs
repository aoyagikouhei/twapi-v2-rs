use crate::responses::{images::Images};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Urls {
    pub display_url: Option<String>, 
    pub description: Option<String>, 
    pub end: Option<i64>, 
    pub expanded_url: Option<String>, 
    pub images: Option<Vec<Images>>, 
    pub media_key: Option<String>, 
    pub start: Option<i64>, 
    pub status: Option<i64>, 
    pub title: Option<String>, 
    pub url: Option<String>, 
    pub unwound_url: Option<String>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Urls {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.images.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true);
        if !res {
          println!("Urls {:?}", self.extra);
        }
        res
    }
}
