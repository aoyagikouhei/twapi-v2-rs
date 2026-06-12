use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum MediaFields {
    #[serde(rename = "alt_text")]
    AltText,
    #[serde(rename = "duration_ms")]
    DurationMs,
    #[serde(rename = "height")]
    Height,
    #[serde(rename = "media_key")]
    MediaKey,
    #[serde(rename = "non_public_metrics")]
    NonPublicMetrics,
    #[serde(rename = "organic_metrics")]
    OrganicMetrics,
    #[serde(rename = "preview_image_url")]
    PreviewImageUrl,
    #[serde(rename = "promoted_metrics")]
    PromotedMetrics,
    #[serde(rename = "public_metrics")]
    PublicMetrics,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "variants")]
    Variants,
    #[serde(rename = "width")]
    Width,
}

impl MediaFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::AltText);
        result.insert(Self::DurationMs);
        result.insert(Self::Height);
        result.insert(Self::MediaKey);
        result.insert(Self::NonPublicMetrics);
        result.insert(Self::OrganicMetrics);
        result.insert(Self::PreviewImageUrl);
        result.insert(Self::PromotedMetrics);
        result.insert(Self::PublicMetrics);
        result.insert(Self::Type);
        result.insert(Self::Url);
        result.insert(Self::Variants);
        result.insert(Self::Width);
        result
    }

    pub fn open() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::AltText);
        result.insert(Self::DurationMs);
        result.insert(Self::Height);
        result.insert(Self::MediaKey);
        result.insert(Self::PreviewImageUrl);
        result.insert(Self::PromotedMetrics);
        result.insert(Self::PublicMetrics);
        result.insert(Self::Type);
        result.insert(Self::Url);
        result.insert(Self::Variants);
        result.insert(Self::Width);
        result
    }
}

impl std::fmt::Display for MediaFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AltText => write!(f, "alt_text"),
            Self::DurationMs => write!(f, "duration_ms"),
            Self::Height => write!(f, "height"),
            Self::MediaKey => write!(f, "media_key"),
            Self::NonPublicMetrics => write!(f, "non_public_metrics"),
            Self::OrganicMetrics => write!(f, "organic_metrics"),
            Self::PreviewImageUrl => write!(f, "preview_image_url"),
            Self::PromotedMetrics => write!(f, "promoted_metrics"),
            Self::PublicMetrics => write!(f, "public_metrics"),
            Self::Type => write!(f, "type"),
            Self::Url => write!(f, "url"),
            Self::Variants => write!(f, "variants"),
            Self::Width => write!(f, "width"),
        }
    }
}

impl Default for MediaFields {
    fn default() -> Self {
        Self::AltText
    }
}
