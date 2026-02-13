use crate::responses::subtitles::Subtitles;
use crate::{
    api::{Authentication, TwapiOptions, apply_options, execute_twitter, make_url},
    error::Error,
    headers::Headers,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/2/media/subtitles/create";

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone, Default)]
pub enum MediaCategory {
    #[serde(rename = "amplify_video")]
    #[default]
    AmplifyVideo,
    #[serde(rename = "tweet_gif")]
    TweetGif,
    #[serde(rename = "tweet_image")]
    TweetImage,
    #[serde(rename = "tweet_video")]
    TweetVideo,
    #[serde(rename = "subtitles")]
    Subtitles,
}

impl std::fmt::Display for MediaCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AmplifyVideo => write!(f, "amplify_video"),
            Self::TweetGif => write!(f, "tweet_gif"),
            Self::TweetImage => write!(f, "tweet_image"),
            Self::TweetVideo => write!(f, "tweet_video"),
            Self::Subtitles => write!(f, "subtitles"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Subtitle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SubtitleInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitles: Option<Vec<Subtitle>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub media_category: MediaCategory,
    pub media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle_info: Option<SubtitleInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    body: Body,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(body: Body) -> Self {
        Self {
            body,
            ..Default::default()
        }
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, URL);
        let builder = client.post(&url).json(&self.body);
        authentication.execute(
            apply_options(builder, &self.twapi_options),
            "POST",
            &url,
            &[],
        )
    }

    pub async fn execute(
        self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Headers), Error> {
        execute_twitter(self.build(authentication)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Response {
    pub data: Subtitles,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() && self.data.is_empty_extra();
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
