use crate::responses::{public_metrics::PublicMetrics, variants::Variants};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Media {
    pub duration_ms: Option<i64>,
    pub height: Option<i64>,
    pub media_key: Option<String>,
    pub preview_image_url: Option<String>,
    pub public_metrics: Option<PublicMetrics>,
    pub r#type: Option<String>,
    pub variants: Option<Vec<Variants>>,
    pub url: Option<String>,
    pub width: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Media {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .public_metrics
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .variants
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Media {:?}", self.extra);
        }
        res
    }
}
