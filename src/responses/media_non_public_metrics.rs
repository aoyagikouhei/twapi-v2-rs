use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct MediaNonPublicMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_0_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_25_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_50_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_75_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_100_count: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl MediaNonPublicMetrics {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("MediaNonPublicMetrics {:?}", self.extra);
        }
        res
    }
}
