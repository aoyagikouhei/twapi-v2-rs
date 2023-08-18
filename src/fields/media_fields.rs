use serde::{Serialize, Deserialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum MediaFields {
    #[serde(rename = "duration_ms")]
    DurationMs,
    #[serde(rename = "height")]
    Height,
    #[serde(rename = "media_key")]
    MediaKey,
    #[serde(rename = "preview_image_url")]
    PreviewImageUrl,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "width")]
    Width,
    #[serde(rename = "public_metrics")]
    PublicMetrics,
    #[serde(rename = "non_public_metrics")]
    NonPublicMetrics,
    #[serde(rename = "organic_metrics")]
    OrganicMetrics,
    #[serde(rename = "promoted_metrics")]
    PromotedMetrics,
    #[serde(rename = "alt_text")]
    AltText,
    #[serde(rename = "variants")]
    Variants,
}

impl MediaFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::DurationMs);
        result.insert(Self::Height);
        result.insert(Self::MediaKey);
        result.insert(Self::PreviewImageUrl);
        result.insert(Self::Type);
        result.insert(Self::Url);
        result.insert(Self::Width);
        result.insert(Self::PublicMetrics);
        result.insert(Self::NonPublicMetrics);
        result.insert(Self::OrganicMetrics);
        result.insert(Self::PromotedMetrics);
        result.insert(Self::AltText);
        result.insert(Self::Variants);
        result
    }

    pub fn open() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::DurationMs);
        result.insert(Self::Height);
        result.insert(Self::MediaKey);
        result.insert(Self::PreviewImageUrl);
        result.insert(Self::Type);
        result.insert(Self::Url);
        result.insert(Self::Width);
        result.insert(Self::PublicMetrics);
        result.insert(Self::AltText);
        result.insert(Self::Variants);
        result
    }
    
}

impl std::fmt::Display for MediaFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DurationMs => write!(f, "duration_ms"),
            Self::Height => write!(f, "height"),
            Self::MediaKey => write!(f, "media_key"),
            Self::PreviewImageUrl => write!(f, "preview_image_url"),
            Self::Type => write!(f, "type"),
            Self::Url => write!(f, "url"),
            Self::Width => write!(f, "width"),
            Self::PublicMetrics => write!(f, "public_metrics"),
            Self::NonPublicMetrics => write!(f, "non_public_metrics"),
            Self::OrganicMetrics => write!(f, "organic_metrics"),
            Self::PromotedMetrics => write!(f, "promoted_metrics"),
            Self::AltText => write!(f, "alt_text"),
            Self::Variants => write!(f, "variants"),
        }
    }
}

impl Default for MediaFields {
    fn default() -> Self { Self::DurationMs }
}