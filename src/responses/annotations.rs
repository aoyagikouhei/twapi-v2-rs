use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Annotations {
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub probability: Option<f64>,
    pub r#type: Option<String>,
    pub normalized_text: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Annotations {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Annotations {:?}", self.extra);
        }
        res
    }
}
