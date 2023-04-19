use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum SpaceFields {
    HostIds,
    CreatedAt,
    CreatorId,
    Id,
    Lang,
    InvitedUserIds,
    ParticipantCount,
    SpeakerIds,
    StartedAt,
    EndedAt,
    SubscriberCount,
    TopicIds,
    State,
    Title,
    UpdatedAt,
    ScheduledStart,
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

impl Default for SpaceFields {
    fn default() -> Self {
        Self::HostIds
    }
}
