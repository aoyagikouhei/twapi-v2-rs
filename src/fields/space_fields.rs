use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
#[derive(Default)]
pub enum SpaceFields {
    #[serde(rename = "host_ids")]
    #[default]
    HostIds,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "creator_id")]
    CreatorId,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "lang")]
    Lang,
    #[serde(rename = "invited_user_ids")]
    InvitedUserIds,
    #[serde(rename = "participant_count")]
    ParticipantCount,
    #[serde(rename = "speaker_ids")]
    SpeakerIds,
    #[serde(rename = "started_at")]
    StartedAt,
    #[serde(rename = "ended_at")]
    EndedAt,
    #[serde(rename = "subscriber_count")]
    SubscriberCount,
    #[serde(rename = "topic_ids")]
    TopicIds,
    #[serde(rename = "state")]
    State,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "updated_at")]
    UpdatedAt,
    #[serde(rename = "scheduled_start")]
    ScheduledStart,
    #[serde(rename = "is_ticketed")]
    IsTicketed,
}

impl SpaceFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::HostIds);
        result.insert(Self::CreatedAt);
        result.insert(Self::CreatorId);
        result.insert(Self::Id);
        result.insert(Self::Lang);
        result.insert(Self::InvitedUserIds);
        result.insert(Self::ParticipantCount);
        result.insert(Self::SpeakerIds);
        result.insert(Self::StartedAt);
        result.insert(Self::EndedAt);
        result.insert(Self::SubscriberCount);
        result.insert(Self::TopicIds);
        result.insert(Self::State);
        result.insert(Self::Title);
        result.insert(Self::UpdatedAt);
        result.insert(Self::ScheduledStart);
        result.insert(Self::IsTicketed);
        result
    }
}

impl std::fmt::Display for SpaceFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HostIds => write!(f, "host_ids"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::CreatorId => write!(f, "creator_id"),
            Self::Id => write!(f, "id"),
            Self::Lang => write!(f, "lang"),
            Self::InvitedUserIds => write!(f, "invited_user_ids"),
            Self::ParticipantCount => write!(f, "participant_count"),
            Self::SpeakerIds => write!(f, "speaker_ids"),
            Self::StartedAt => write!(f, "started_at"),
            Self::EndedAt => write!(f, "ended_at"),
            Self::SubscriberCount => write!(f, "subscriber_count"),
            Self::TopicIds => write!(f, "topic_ids"),
            Self::State => write!(f, "state"),
            Self::Title => write!(f, "title"),
            Self::UpdatedAt => write!(f, "updated_at"),
            Self::ScheduledStart => write!(f, "scheduled_start"),
            Self::IsTicketed => write!(f, "is_ticketed"),
        }
    }
}

