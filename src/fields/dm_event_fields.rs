use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum DmEventFields {
    DurationMs,
    Height,
    MediaKey,
    PreviewImageUrl,
    Type,
    Url,
    Width,
    PublicMetrics,
    AltText,
    Variants,
}

impl DmEventFields {
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
        result.insert(Self::AltText);
        result.insert(Self::Variants);
        result
    }
}

impl std::fmt::Display for DmEventFields {
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
            Self::AltText => write!(f, "alt_text"),
            Self::Variants => write!(f, "variants"),
        }
    }
}

impl Default for DmEventFields {
    fn default() -> Self {
        Self::DurationMs
    }
}
