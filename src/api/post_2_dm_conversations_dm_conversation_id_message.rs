use super::{execute_twitter, TwitterResult};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/dm_conversations/:dm_conversation_id/messages";

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
    dm_conversation_id: String,
    body: Body,
}

impl Api {
    pub fn new(bearer_code: &str, dm_conversation_id: &str, body: Body) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            dm_conversation_id: dm_conversation_id.to_owned(),
            body,
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(URL.replace(":dm_conversation_id", &self.dm_conversation_id))
            .bearer_auth(self.bearer_code)
            .json(&serde_json::to_value(&self.body).unwrap())
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}
