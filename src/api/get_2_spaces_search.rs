use crate::fields::{
    space_fields::SpaceFields, topic_fields::TopicFields, user_fields::UserFields,
};
use crate::responses::{errors::Errors, includes::Includes, meta::Meta, spaces::Spaces};
use crate::{
    api::{execute_twitter, Auth},
    error::Error,
    rate_limit::RateLimit,
};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/spaces/search";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    #[serde(rename = "invited_user_ids")]
    InvitedUserIds,
    #[serde(rename = "speaker_ids")]
    SpeakerIds,
    #[serde(rename = "creator_id")]
    CreatorId,
    #[serde(rename = "host_ids")]
    HostIds,
    #[serde(rename = "topics_ids")]
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
    #[serde(rename = "all")]
    All,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "scheduled")]
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
    query: String,
    expansions: Option<HashSet<Expansions>>,
    space_fields: Option<HashSet<SpaceFields>>,
    state: Option<State>,
    topic_fields: Option<HashSet<TopicFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_owned(),
            ..Default::default()
        }
    }

    pub fn all(query: &str) -> Self {
        Self {
            query: query.to_owned(),
            expansions: Some(Expansions::all()),
            space_fields: Some(SpaceFields::all()),
            topic_fields: Some(TopicFields::all()),
            user_fields: Some(UserFields::all()),
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

    pub fn build(self, auth: &impl Auth) -> RequestBuilder {
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
        let builder = client.get(URL).query(&query_parameters);
        auth.auth(
            builder,
            "get",
            URL,
            &query_parameters
                .iter()
                .map(|it| (it.0, it.1.as_str()))
                .collect(),
        )
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(auth)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<Spaces>>,
    pub errors: Option<Vec<Errors>>,
    pub includes: Option<Includes>,
    pub meta: Option<Meta>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .data
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .errors
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true)
            && self
                .includes
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .meta
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
