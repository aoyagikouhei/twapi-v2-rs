use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum UserFields {
    CreatedAt,
    Description,
    Entities,
    Id,
    Location,
    Name,
    PinnedTweetId,
    ProfileImageUrl,
    Protected,
    PublicMetrics,
    Url,
    Username,
    Verified,
    VerifiedType,
    Withheld,
}

impl UserFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::CreatedAt);
        result.insert(Self::Description);
        result.insert(Self::Entities);
        result.insert(Self::Id);
        result.insert(Self::Location);
        result.insert(Self::Name);
        result.insert(Self::PinnedTweetId);
        result.insert(Self::ProfileImageUrl);
        result.insert(Self::Protected);
        result.insert(Self::PublicMetrics);
        result.insert(Self::Url);
        result.insert(Self::Username);
        result.insert(Self::Verified);
        result.insert(Self::VerifiedType);
        result.insert(Self::Withheld);
        result
    }

    pub fn open() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::CreatedAt);
        result.insert(Self::Description);
        result.insert(Self::Entities);
        result.insert(Self::Id);
        result.insert(Self::Location);
        result.insert(Self::Name);
        result.insert(Self::PinnedTweetId);
        result.insert(Self::ProfileImageUrl);
        result.insert(Self::Protected);
        result.insert(Self::PublicMetrics);
        result.insert(Self::Url);
        result.insert(Self::Username);
        result.insert(Self::Verified);
        result.insert(Self::Withheld);
        result
    }
}

impl std::fmt::Display for UserFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::Description => write!(f, "description"),
            Self::Entities => write!(f, "entities"),
            Self::Id => write!(f, "id"),
            Self::Location => write!(f, "location"),
            Self::Name => write!(f, "name"),
            Self::PinnedTweetId => write!(f, "pinned_tweet_id"),
            Self::ProfileImageUrl => write!(f, "profile_image_url"),
            Self::Protected => write!(f, "protected"),
            Self::PublicMetrics => write!(f, "public_metrics"),
            Self::Url => write!(f, "url"),
            Self::Username => write!(f, "username"),
            Self::Verified => write!(f, "verified"),
            Self::VerifiedType => write!(f, "verified_type"),
            Self::Withheld => write!(f, "withheld"),
        }
    }
}

impl Default for UserFields {
    fn default() -> Self {
        Self::CreatedAt
    }
}
