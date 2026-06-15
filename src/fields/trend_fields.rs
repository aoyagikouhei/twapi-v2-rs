use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum TrendFields {
    #[serde(rename = "trend_name")]
    TrendName,
    #[serde(rename = "tweet_count")]
    TweetCount,
}

impl TrendFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::TrendName);
        result.insert(Self::TweetCount);
        result
    }
}

impl std::fmt::Display for TrendFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TrendName => write!(f, "trend_name"),
            Self::TweetCount => write!(f, "tweet_count"),
        }
    }
}

impl Default for TrendFields {
    fn default() -> Self {
        Self::TrendName
    }
}
