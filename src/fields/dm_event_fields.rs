use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum DmEventFields {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "event_type")]
    EventType,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "dm_conversation_id")]
    DmConversationId,
    #[serde(rename = "sender_id")]
    SenderId,
    #[serde(rename = "participant_ids")]
    ParticipantIds,
    #[serde(rename = "referenced_tweets")]
    ReferencedTweets,
    #[serde(rename = "attachments")]
    Attachments,
}

impl DmEventFields {
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

impl std::fmt::Display for DmEventFields {
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

impl Default for DmEventFields {
    fn default() -> Self {
        Self::Id
    }
}
