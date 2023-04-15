use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PollFields {
    DurationMinutes,
    EndDatetime,
    Id,
    Options,
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
