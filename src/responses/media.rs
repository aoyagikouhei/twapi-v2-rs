use crate::responses::{
    media_non_public_metrics::MediaNonPublicMetrics, media_organic_metrics::MediaOrganicMetrics,
    media_promoted_metrics::MediaPromotedMetrics, media_public_metrics::MediaPublicMetrics,
    variants::Variants,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Media {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_public_metrics: Option<MediaNonPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organic_metrics: Option<MediaOrganicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoted_metrics: Option<MediaPromotedMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<MediaPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<Variants>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Media {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .non_public_metrics
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .organic_metrics
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .promoted_metrics
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
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
