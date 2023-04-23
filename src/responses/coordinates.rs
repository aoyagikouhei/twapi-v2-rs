use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Coordinates {
    pub r#type: String, 
    pub coordinates: Option<Vec<f64>>, 
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
