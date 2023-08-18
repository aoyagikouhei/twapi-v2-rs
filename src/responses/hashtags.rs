use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Hashtags {
    pub start: Option<i64>, 
    pub end: Option<i64>, 
    pub hashtag: Option<String>, 
    pub tag: Option<String>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Hashtags {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Hashtags {:?}", self.extra);
        }
        res
    }
}
