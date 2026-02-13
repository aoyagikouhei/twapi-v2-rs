use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
#[derive(Default)]
pub enum UserFields {
    #[serde(rename = "affiliation")]
    #[default]
    Affiliation,
    #[serde(rename = "connection_status")]
    ConnectionStatus,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "entities")]
    Entities,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "is_identity_verified")]
    IsIdentityVerified,
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "most_recent_tweet_id")]
    MostRecentTweetId,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "pinned_tweet_id")]
    PinnedTweetId,
    #[serde(rename = "profile_banner_url")]
    ProfileBannerUrl,
    #[serde(rename = "profile_image_url")]
    ProfileImageUrl,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "public_metrics")]
    PublicMetrics,
    #[serde(rename = "receives_your_dm")]
    ReceivesYourDm,
    #[serde(rename = "subscription_type")]
    SubscriptionType,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "username")]
    Username,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "verified_followers_count")]
    VerifiedFollowersCount,
    #[serde(rename = "verified_type")]
    VerifiedType,
    #[serde(rename = "withheld")]
    Withheld,
}

impl UserFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Affiliation);
        result.insert(Self::ConnectionStatus);
        result.insert(Self::CreatedAt);
        result.insert(Self::Description);
        result.insert(Self::Entities);
        result.insert(Self::Id);
        result.insert(Self::IsIdentityVerified);
        result.insert(Self::Location);
        result.insert(Self::MostRecentTweetId);
        result.insert(Self::Name);
        result.insert(Self::PinnedTweetId);
        result.insert(Self::ProfileBannerUrl);
        result.insert(Self::ProfileImageUrl);
        result.insert(Self::Protected);
        result.insert(Self::PublicMetrics);
        result.insert(Self::ReceivesYourDm);
        result.insert(Self::SubscriptionType);
        result.insert(Self::Url);
        result.insert(Self::Username);
        result.insert(Self::Verified);
        result.insert(Self::VerifiedFollowersCount);
        result.insert(Self::VerifiedType);
        result.insert(Self::Withheld);
        result
    }
}

impl std::fmt::Display for UserFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Affiliation => write!(f, "affiliation"),
            Self::ConnectionStatus => write!(f, "connection_status"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::Description => write!(f, "description"),
            Self::Entities => write!(f, "entities"),
            Self::Id => write!(f, "id"),
            Self::IsIdentityVerified => write!(f, "is_identity_verified"),
            Self::Location => write!(f, "location"),
            Self::MostRecentTweetId => write!(f, "most_recent_tweet_id"),
            Self::Name => write!(f, "name"),
            Self::PinnedTweetId => write!(f, "pinned_tweet_id"),
            Self::ProfileBannerUrl => write!(f, "profile_banner_url"),
            Self::ProfileImageUrl => write!(f, "profile_image_url"),
            Self::Protected => write!(f, "protected"),
            Self::PublicMetrics => write!(f, "public_metrics"),
            Self::ReceivesYourDm => write!(f, "receives_your_dm"),
            Self::SubscriptionType => write!(f, "subscription_type"),
            Self::Url => write!(f, "url"),
            Self::Username => write!(f, "username"),
            Self::Verified => write!(f, "verified"),
            Self::VerifiedFollowersCount => write!(f, "verified_followers_count"),
            Self::VerifiedType => write!(f, "verified_type"),
            Self::Withheld => write!(f, "withheld"),
        }
    }
}

