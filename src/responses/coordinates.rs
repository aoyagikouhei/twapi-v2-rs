use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Coordinates {
    pub r#type: String, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Vec<f64>>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Coordinates {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("Coordinates {:?}", self.extra);
        }
        res
    }
}
