use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
#[derive(Default)]
pub enum TopicFields {
    #[serde(rename = "id")]
    #[default]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "description")]
    Description,
}

impl TopicFields {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::Id);
        result.insert(Self::Name);
        result.insert(Self::Description);
        result
    }
}

impl std::fmt::Display for TopicFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::Description => write!(f, "description"),
        }
    }
}

