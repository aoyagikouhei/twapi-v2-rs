use crate::responses::coordinates::Coordinates;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Geo {
    pub r#type: String,
    pub bbox: Option<Vec<f64>>,
    pub contained_within: Option<Vec<String>>,
    pub coordinates: Option<Coordinates>,
    pub place_id: Option<String>,
    pub properties: Option<serde_json::Value>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
