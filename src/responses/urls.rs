use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Urls {
    pub display_url: Option<String>,
    pub end: Option<i64>,
    pub expanded_url: Option<String>,
    pub media_key: Option<String>,
    pub start: Option<i64>,
    pub url: Option<String>,
}
