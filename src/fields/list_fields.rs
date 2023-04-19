use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum ListFields {
    CreatedAt,
    FollowerCount,
    MemberCount,
    Private,
    Description,
    OwnerId,
}

impl ListFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::CreatedAt);
        result.insert(Self::FollowerCount);
        result.insert(Self::MemberCount);
        result.insert(Self::Private);
        result.insert(Self::Description);
        result.insert(Self::OwnerId);
        result
    }
}

impl std::fmt::Display for ListFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::FollowerCount => write!(f, "follower_count"),
            Self::MemberCount => write!(f, "member_count"),
            Self::Private => write!(f, "private"),
            Self::Description => write!(f, "description"),
            Self::OwnerId => write!(f, "owner_id"),
        }
    }
}

impl Default for ListFields {
    fn default() -> Self {
        Self::CreatedAt
    }
}
