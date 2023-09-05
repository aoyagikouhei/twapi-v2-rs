use crate::responses::errors::Errors;
use crate::{
    api::{execute_twitter, Authentication},
    error::Error,
    rate_limit::RateLimit,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.twitter.com/2/tweets";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Geo {
    pub place_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Media {
    pub media_ids: Vec<String>,
    pub tagged_user_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Poll {
    pub duration_minutes: String,
    pub options: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Reply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_reply_user_ids: Option<Vec<String>>,
    pub in_reply_to_tweet_id: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum ReplySettings {
    #[serde(rename = "mentionedUsers")]
    Mentionedusers,
    #[serde(rename = "following")]
    Following,
}

impl std::fmt::Display for ReplySettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Mentionedusers => write!(f, "mentionedUsers"),
            Self::Following => write!(f, "following"),
        }
    }
}

impl Default for ReplySettings {
    fn default() -> Self {
        Self::Mentionedusers
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_deep_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_super_followers_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_tweet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<Reply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<ReplySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    body: Body,
}

impl Api {
    pub fn new(body: Body) -> Self {
        Self { body }
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = URL.to_string();
        let builder = client.post(&url).json(&self.body);
        authentication.execute(builder, "POST", &url, &[])
    }

    pub async fn execute(
        self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(authentication)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Data>,
    pub errors: Option<Vec<Errors>>,
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
                .unwrap_or(true)
            && self
                .errors
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    pub id: Option<String>,
    pub text: Option<String>,
    pub edit_history_tweet_ids: Option<Vec<String>>,
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
