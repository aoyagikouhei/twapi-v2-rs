use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone, Default)]
pub enum UsageFields {
    #[serde(rename = "daily_client_app_usage")]
    #[default]
    DailyClientAppUsage,
    #[serde(rename = "daily_project_usage")]
    DailyProjectUsage,
}

impl UsageFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::DailyClientAppUsage);
        result.insert(Self::DailyProjectUsage);
        result
    }
}

impl std::fmt::Display for UsageFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DailyClientAppUsage => write!(f, "daily_client_app_usage"),
            Self::DailyProjectUsage => write!(f, "daily_project_usage"),
        }
    }
}
