use crate::responses::{entities::Entities};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct NoteTweet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Entities>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl NoteTweet {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.entities.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("NoteTweet {:?}", self.extra);
        }
        res
    }
}
