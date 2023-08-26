use crate::{
    api::{execute_twitter, Auth},
    error::Error,
    rate_limit::RateLimit,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/dm_conversations/with/:participant_id/messages";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Attachment {
    pub media_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    participant_id: String,
    body: Body,
}

impl Api {
    pub fn new(participant_id: &str, body: Body) -> Self {
        Self {
            participant_id: participant_id.to_owned(),
            body,
        }
    }

    pub fn build(self, auth: &impl Auth) -> RequestBuilder {
        let client = reqwest::Client::new();
        let builder = client
            .post(URL.replace(":participant_id", &self.participant_id))
            .json(&self.body);
        auth.auth(builder, "POST", URL, &vec![])
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(auth)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Data>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .data
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub dm_conversation_id: Option<String>,
    pub dm_event_id: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
