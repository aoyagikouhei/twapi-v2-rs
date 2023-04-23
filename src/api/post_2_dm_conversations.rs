use crate::{api::execute_twitter, error::Error, rate_limit::RateLimit};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/dm_conversations";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum ConversationType {
    Group,
}

impl std::fmt::Display for ConversationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Group => write!(f, "Group"),
        }
    }
}

impl Default for ConversationType {
    fn default() -> Self {
        Self::Group
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Attachment {
    media_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Message {
    attachments: Option<Vec<Attachment>>,
    text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    conversation_type: ConversationType,
    participant_ids: Vec<String>,
    message: Message,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    body: Body,
}

impl Api {
    pub fn new(bearer_code: &str, body: Body) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            body,
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(URL)
            .bearer_auth(self.bearer_code)
            .json(&serde_json::to_value(&self.body).unwrap())
    }

    pub async fn execute(self) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build()).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Data>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub dm_conversation_id: Option<String>,
    pub dm_event_id: Option<String>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}
