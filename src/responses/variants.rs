use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Variants {
    pub bit_rate: Option<i64>, 
    pub content_type: Option<String>, 
    pub url: Option<String>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Variants {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Variants {:?}", self.extra);
        }
        res
    }
}
