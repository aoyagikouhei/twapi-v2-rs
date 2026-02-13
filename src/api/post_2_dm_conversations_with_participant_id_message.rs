use serde::{Serialize, Deserialize};
use reqwest::RequestBuilder;
use crate::{error::Error, headers::Headers, api::{apply_options, execute_twitter, Authentication, make_url, TwapiOptions}};

const URL: &str = "/2/dm_conversations/with/:participant_id/messages";

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
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(participant_id: &str, body: Body) -> Self {
        Self {
            participant_id: participant_id.to_owned(),
            body,
            ..Default::default()
        }
    }
    
    
    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, &URL.replace(":participant_id", &self.participant_id));
        let builder = client
            .post(&url)
            .json(&self.body)
        ;
        authentication.execute(apply_options(builder, &self.twapi_options), "POST", &url, &[])
    }

    pub async fn execute(&self, authentication: &impl Authentication) -> Result<(Response, Headers), Error> {
        execute_twitter(|| self.build(authentication), &self.twapi_options).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
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



#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_conversation_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
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
