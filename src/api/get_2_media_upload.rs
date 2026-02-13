use crate::responses::{errors::Errors, includes::Includes, media_upload::MediaUpload};
use crate::{
    api::{Authentication, TwapiOptions, execute_twitter, make_url},
    error::Error,
    headers::Headers,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/2/media/upload";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
#[derive(Default)]
pub enum Command {
    #[serde(rename = "STATUS")]
    #[default]
    Status,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Status => write!(f, "STATUS"),
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct Api {
    media_id: String,
    command: Command,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(media_id: &str, command: Command) -> Self {
        Self {
            media_id: media_id.to_owned(),
            command,
            ..Default::default()
        }
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        let query_parameters = vec![
            ("media_id", self.media_id.to_string()),
            ("command", self.command.to_string()),
        ];
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, URL);
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
    pub data: Option<MediaUpload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Errors>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Includes>,
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
                .unwrap_or(true)
            && self
                .includes
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
