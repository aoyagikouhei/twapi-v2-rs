use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Summary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_created: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_deleted: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<i64>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Summary {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Summary {:?}", self.extra);
        }
        res
    }
}
