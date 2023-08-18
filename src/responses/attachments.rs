use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Attachments {
    pub media_keys: Option<Vec<String>>, 
    pub card_ids: Option<Vec<String>>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Attachments {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Attachments {:?}", self.extra);
        }
        res
    }
}
