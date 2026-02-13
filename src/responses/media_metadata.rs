use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct MediaMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_key: Option<String>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl MediaMetadata {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("MediaMetadata {:?}", self.extra);
        }
        res
    }
}
