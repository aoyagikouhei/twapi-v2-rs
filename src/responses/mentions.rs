use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Mentions {
    pub start: Option<i64>, 
    pub end: Option<i64>, 
    pub username: Option<String>, 
    pub id: Option<String>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Mentions {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Mentions {:?}", self.extra);
        }
        res
    }
}
