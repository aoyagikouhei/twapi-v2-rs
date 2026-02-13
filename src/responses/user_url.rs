use crate::responses::{urls::Urls};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct UserUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<Urls>>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UserUrl {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.urls.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true);
        if !res {
          println!("UserUrl {:?}", self.extra);
        }
        res
    }
}
