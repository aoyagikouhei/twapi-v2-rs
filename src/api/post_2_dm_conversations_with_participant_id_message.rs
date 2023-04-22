use super::{execute_twitter, TwitterResult};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/dm_conversations/with/:participant_id/messages";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Attachment {
    media_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    attachments: Option<Vec<Attachment>>,
    text: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    participant_id: String,
    body: Body,
}

impl Api {
    pub fn new(bearer_code: &str, participant_id: &str, body: Body) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            participant_id: participant_id.to_owned(),
            body,
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(URL.replace(":participant_id", &self.participant_id))
            .bearer_auth(self.bearer_code)
            .json(&serde_json::to_value(&self.body).unwrap())
    }

    pub async fn execute(self) -> TwitterResult {
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
