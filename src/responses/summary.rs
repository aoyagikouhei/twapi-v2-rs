use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Summary {
    pub created: Option<i64>,
    pub not_created: Option<i64>,
    pub deleted: Option<i64>,
    pub not_deleted: Option<i64>,
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
