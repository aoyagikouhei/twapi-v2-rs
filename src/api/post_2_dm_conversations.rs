use serde::{Serialize, Deserialize};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

const URL: &str = "https://api.twitter.com/2/dm_conversations";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum ConversationType {
    #[serde(rename = "Group")]
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
    fn default() -> Self { Self::Group }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Attachment {
    pub media_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub conversation_type: ConversationType,
    pub participant_ids: Vec<String>,
    pub message: Message,
}



#[derive(Debug, Clone, Default)]
pub struct Api {
    body: Body,
}

impl Api {
    pub fn new(body: Body) -> Self {
        Self {
            body,
        }
    }
    
    pub fn build(self) -> RequestBuilder {
        
        let client = reqwest::Client::new();
        client
            .post(URL)
            .json(&self.body)
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(), auth).await
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
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
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
