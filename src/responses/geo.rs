use crate::responses::{coordinates::Coordinates};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Geo {
    pub r#type: Option<String>, 
    pub bbox: Option<Vec<f64>>, 
    pub contained_within: Option<Vec<String>>, 
    pub coordinates: Option<Coordinates>, 
    pub place_id: Option<String>, 
    pub properties: Option<serde_json::Value>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Geo {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.coordinates.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Geo {:?}", self.extra);
        }
        res
    }
}
