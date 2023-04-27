use crate::fields::{
    dm_event_fields::DmEventFields, media_fields::MediaFields, tweet_fields::TweetFields,
    user_fields::UserFields,
};
use crate::responses::{dm_events::DmEvents, errors::Errors, includes::Includes, meta::Meta};
use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://api.twitter.com/2/dm_conversations/with/:participant_id/dm_events";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum EventTypes {
    Messagecreate,
    Participantsjoin,
    Participantsleave,
}

impl std::fmt::Display for EventTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Messagecreate => write!(f, "MessageCreate"),
            Self::Participantsjoin => write!(f, "ParticipantsJoin"),
            Self::Participantsleave => write!(f, "ParticipantsLeave"),
        }
    }
}

impl Default for EventTypes {
    fn default() -> Self {
        Self::Messagecreate
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    AttachmentsMediaKeys,
    ReferencedTweetsId,
    SenderId,
    ParticipantIds,
}

impl Expansions {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::AttachmentsMediaKeys);
        result.insert(Self::ReferencedTweetsId);
        result.insert(Self::SenderId);
        result.insert(Self::ParticipantIds);
        result
    }
}

impl std::fmt::Display for Expansions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AttachmentsMediaKeys => write!(f, "attachments.media_keys"),
            Self::ReferencedTweetsId => write!(f, "referenced_tweets.id"),
            Self::SenderId => write!(f, "sender_id"),
            Self::ParticipantIds => write!(f, "participant_ids"),
        }
    }
}

impl Default for Expansions {
    fn default() -> Self {
        Self::AttachmentsMediaKeys
    }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    participant_id: String,
    dm_event_fields: Option<HashSet<DmEventFields>>,
    event_types: Option<EventTypes>,
    expansions: Option<HashSet<Expansions>>,
    max_results: Option<usize>,
    media_fields: Option<HashSet<MediaFields>>,
    pagination_token: Option<String>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(bearer_code: &str, participant_id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            participant_id: participant_id.to_owned(),
            ..Default::default()
        }
    }

    pub fn all(bearer_code: &str, participant_id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            participant_id: participant_id.to_owned(),
            dm_event_fields: Some(DmEventFields::all()),
            expansions: Some(Expansions::all()),
            media_fields: Some(MediaFields::all()),
            tweet_fields: Some(TweetFields::organic()),
            user_fields: Some(UserFields::all()),
            max_results: Some(100),
            ..Default::default()
        }
    }

    pub fn open(bearer_code: &str, participant_id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            participant_id: participant_id.to_owned(),
            dm_event_fields: Some(DmEventFields::all()),
            expansions: Some(Expansions::all()),
            media_fields: Some(MediaFields::open()),
            tweet_fields: Some(TweetFields::open()),
            user_fields: Some(UserFields::all()),
            max_results: Some(100),
            ..Default::default()
        }
    }

    pub fn dm_event_fields(mut self, value: HashSet<DmEventFields>) -> Self {
        self.dm_event_fields = Some(value);
        self
    }

    pub fn event_types(mut self, value: EventTypes) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn expansions(mut self, value: HashSet<Expansions>) -> Self {
        self.expansions = Some(value);
        self
    }

    pub fn max_results(mut self, value: usize) -> Self {
        self.max_results = Some(value);
        self
    }

    pub fn media_fields(mut self, value: HashSet<MediaFields>) -> Self {
        self.media_fields = Some(value);
        self
    }

    pub fn pagination_token(mut self, value: &str) -> Self {
        self.pagination_token = Some(value.to_owned());
        self
    }

    pub fn tweet_fields(mut self, value: HashSet<TweetFields>) -> Self {
        self.tweet_fields = Some(value);
        self
    }

    pub fn user_fields(mut self, value: HashSet<UserFields>) -> Self {
        self.user_fields = Some(value);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(dm_event_fields) = self.dm_event_fields {
            query_parameters.push(("dm_event.fields", dm_event_fields.iter().join(",")));
        }
        if let Some(event_types) = self.event_types {
            query_parameters.push(("event_types", event_types.to_string()));
        }
        if let Some(expansions) = self.expansions {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(max_results) = self.max_results {
            query_parameters.push(("max_results", max_results.to_string()));
        }
        if let Some(media_fields) = self.media_fields {
            query_parameters.push(("media.fields", media_fields.iter().join(",")));
        }
        if let Some(pagination_token) = self.pagination_token {
            query_parameters.push(("pagination_token", pagination_token));
        }
        if let Some(tweet_fields) = self.tweet_fields {
            query_parameters.push(("tweet.fields", tweet_fields.iter().join(",")));
        }
        if let Some(user_fields) = self.user_fields {
            query_parameters.push(("user.fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        client
            .get(URL.replace(":participant_id", &self.participant_id))
            .query(&query_parameters)
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<DmEvents>>,
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
