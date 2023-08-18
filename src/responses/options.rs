use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Options {
    pub position: Option<i64>, 
    pub label: Option<String>, 
    pub votes: Option<i64>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Options {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Options {:?}", self.extra);
        }
        res
    }
}
