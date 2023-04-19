use super::{execute_twitter, TwitterResult};
use crate::fields::{
    space_fields::SpaceFields, topic_fields::TopicFields, user_fields::UserFields,
};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/spaces/search";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    InvitedUserIds,
    SpeakerIds,
    CreatorId,
    HostIds,
    TopicsIds,
}

impl Expansions {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::InvitedUserIds);
        result.insert(Self::SpeakerIds);
        result.insert(Self::CreatorId);
        result.insert(Self::HostIds);
        result.insert(Self::TopicsIds);
        result
    }
}

impl std::fmt::Display for Expansions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvitedUserIds => write!(f, "invited_user_ids"),
            Self::SpeakerIds => write!(f, "speaker_ids"),
            Self::CreatorId => write!(f, "creator_id"),
            Self::HostIds => write!(f, "host_ids"),
            Self::TopicsIds => write!(f, "topics_ids"),
        }
    }
}

impl Default for Expansions {
    fn default() -> Self {
        Self::InvitedUserIds
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum State {
    All,
    Live,
    Scheduled,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Live => write!(f, "live"),
            Self::Scheduled => write!(f, "scheduled"),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    query: String,
    expansions: Option<HashSet<Expansions>>,
    space_fields: Option<HashSet<SpaceFields>>,
    state: Option<State>,
    topic_fields: Option<HashSet<TopicFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(bearer_code: &str, query: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            query: query.to_owned(),
            ..Default::default()
        }
    }

    pub fn expansions(mut self, value: HashSet<Expansions>) -> Self {
        self.expansions = Some(value);
        self
    }

    pub fn space_fields(mut self, value: HashSet<SpaceFields>) -> Self {
        self.space_fields = Some(value);
        self
    }

    pub fn state(mut self, value: State) -> Self {
        self.state = Some(value);
        self
    }

    pub fn topic_fields(mut self, value: HashSet<TopicFields>) -> Self {
        self.topic_fields = Some(value);
        self
    }

    pub fn user_fields(mut self, value: HashSet<UserFields>) -> Self {
        self.user_fields = Some(value);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("query", self.query));
        if let Some(expansions) = self.expansions {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(space_fields) = self.space_fields {
            query_parameters.push(("space.fields", space_fields.iter().join(",")));
        }
        if let Some(state) = self.state {
            query_parameters.push(("state", state.to_string()));
        }
        if let Some(topic_fields) = self.topic_fields {
            query_parameters.push(("topic.fields", topic_fields.iter().join(",")));
        }
        if let Some(user_fields) = self.user_fields {
            query_parameters.push(("user.fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        client
            .get(URL)
            .query(&query_parameters)
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}
