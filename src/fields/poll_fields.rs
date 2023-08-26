use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum PollFields {
    #[serde(rename = "duration_minutes")]
    DurationMinutes,
    #[serde(rename = "end_datetime")]
    EndDatetime,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "options")]
    Options,
    #[serde(rename = "voting_status")]
    VotingStatus,
}

impl PollFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::DurationMinutes);
        result.insert(Self::EndDatetime);
        result.insert(Self::Id);
        result.insert(Self::Options);
        result.insert(Self::VotingStatus);
        result
    }
}

impl std::fmt::Display for PollFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DurationMinutes => write!(f, "duration_minutes"),
            Self::EndDatetime => write!(f, "end_datetime"),
            Self::Id => write!(f, "id"),
            Self::Options => write!(f, "options"),
            Self::VotingStatus => write!(f, "voting_status"),
        }
    }
}

impl Default for PollFields {
    fn default() -> Self {
        Self::DurationMinutes
    }
}
