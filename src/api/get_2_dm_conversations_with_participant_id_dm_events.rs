use crate::fields::{
    dm_event_fields::DmEventFields, media_fields::MediaFields, tweet_fields::TweetFields,
    user_fields::UserFields,
};
use crate::responses::{dm_events::DmEvents, errors::Errors, includes::Includes, meta::Meta};
use crate::{
    api::{Authentication, TwapiOptions, execute_twitter, make_url},
    error::Error,
    headers::Headers,
};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "/2/dm_conversations/with/:participant_id/dm_events";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
#[derive(Default)]
pub enum EventTypes {
    #[serde(rename = "MessageCreate")]
    #[default]
    Messagecreate,
    #[serde(rename = "ParticipantsJoin")]
    Participantsjoin,
    #[serde(rename = "ParticipantsLeave")]
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


#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
#[derive(Default)]
pub enum Expansions {
    #[serde(rename = "attachments.media_keys")]
    #[default]
    AttachmentsMediaKeys,
    #[serde(rename = "referenced_tweets.id")]
    ReferencedTweetsId,
    #[serde(rename = "sender_id")]
    SenderId,
    #[serde(rename = "participant_ids")]
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


#[derive(Debug, Clone, Default)]
pub struct Api {
    participant_id: String,
    dm_event_fields: Option<HashSet<DmEventFields>>,
    event_types: Option<EventTypes>,
    expansions: Option<HashSet<Expansions>>,
    max_results: Option<usize>,
    media_fields: Option<HashSet<MediaFields>>,
    pagination_token: Option<String>,
    tweet_fields: Option<HashSet<TweetFields>>,
    user_fields: Option<HashSet<UserFields>>,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(participant_id: &str) -> Self {
        Self {
            participant_id: participant_id.to_owned(),
            ..Default::default()
        }
    }

    pub fn all(participant_id: &str) -> Self {
        Self {
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

    pub fn open(participant_id: &str) -> Self {
        Self {
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

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(dm_event_fields) = self.dm_event_fields.as_ref() {
            query_parameters.push(("dm_event.fields", dm_event_fields.iter().join(",")));
        }
        if let Some(event_types) = self.event_types.as_ref() {
            query_parameters.push(("event_types", event_types.to_string()));
        }
        if let Some(expansions) = self.expansions.as_ref() {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(max_results) = self.max_results.as_ref() {
            query_parameters.push(("max_results", max_results.to_string()));
        }
        if let Some(media_fields) = self.media_fields.as_ref() {
            query_parameters.push(("media.fields", media_fields.iter().join(",")));
        }
        if let Some(pagination_token) = self.pagination_token.as_ref() {
            query_parameters.push(("pagination_token", pagination_token.to_string()));
        }
        if let Some(tweet_fields) = self.tweet_fields.as_ref() {
            query_parameters.push(("tweet.fields", tweet_fields.iter().join(",")));
        }
        if let Some(user_fields) = self.user_fields.as_ref() {
            query_parameters.push(("user.fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        let url = make_url(
            &self.twapi_options,
            &URL.replace(":participant_id", &self.participant_id),
        );
        let builder = client.get(&url).query(&query_parameters);
        authentication.execute(
            builder,
            "GET",
            &url,
            &query_parameters
                .iter()
                .map(|it| (it.0, it.1.as_str()))
                .collect::<Vec<_>>(),
        )
    }

    pub async fn execute(
        &self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Headers), Error> {
        execute_twitter(|| self.build(authentication), &self.twapi_options).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DmEvents>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Errors>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Includes>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
