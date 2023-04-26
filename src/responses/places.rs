use crate::responses::geo::Geo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Places {
    pub full_name: String,
    pub id: String,
    pub contained_within: Option<Vec<String>>,
    pub country: Option<String>,
    pub country_code: Option<i64>,
    pub geo: Option<Geo>,
    pub name: Option<String>,
    pub place_type: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Places {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .geo
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Places {:?}", self.extra);
        }
        res
    }
}
