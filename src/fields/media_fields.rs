use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum MediaFields {
    Id,
    Text,
    EventType,
    CreatedAt,
    DmConversationId,
    SenderId,
    ParticipantIds,
    ReferencedTweets,
    Attachments,
}

impl MediaFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Id);
        result.insert(Self::Text);
        result.insert(Self::EventType);
        result.insert(Self::CreatedAt);
        result.insert(Self::DmConversationId);
        result.insert(Self::SenderId);
        result.insert(Self::ParticipantIds);
        result.insert(Self::ReferencedTweets);
        result.insert(Self::Attachments);
        result
    }
}

impl std::fmt::Display for MediaFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Text => write!(f, "text"),
            Self::EventType => write!(f, "event_type"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::DmConversationId => write!(f, "dm_conversation_id"),
            Self::SenderId => write!(f, "sender_id"),
            Self::ParticipantIds => write!(f, "participant_ids"),
            Self::ReferencedTweets => write!(f, "referenced_tweets"),
            Self::Attachments => write!(f, "attachments"),
        }
    }
}

impl Default for MediaFields {
    fn default() -> Self {
        Self::Id
    }
}
