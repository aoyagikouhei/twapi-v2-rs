use crate::{
    api::{Authentication, TwapiOptions, execute_twitter, make_url},
    error::Error,
    headers::Headers,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/2/users/:id/retweets/:source_tweet_id";

#[derive(Debug, Clone, Default)]
pub struct Api {
    id: String,
    source_tweet_id: String,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(id: &str, source_tweet_id: &str) -> Self {
        Self {
            id: id.to_owned(),
            source_tweet_id: source_tweet_id.to_owned(),
            ..Default::default()
        }
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = make_url(
            &self.twapi_options,
            &URL.replace(":id", &self.id)
                .replace(":source_tweet_id", &self.source_tweet_id),
        );
        let builder = client.delete(&url);
        authentication.execute(builder, "DELETE", &url, &[])
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

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retweeted: Option<bool>,
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
