use crate::{
    api::{Authentication, TwapiOptions},
    error::Error,
    headers::Headers,
    upload::{execute_no_response, make_url},
};
use reqwest::RequestBuilder;
use serde::Serialize;

const URL: &str = "/1.1/media/subtitles/create.json";

#[derive(Serialize, Debug, Clone, Default)]
pub struct Subtitle {
    pub media_id: String,
    pub language_code: String,
    pub display_name: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct SubtitleInfo {
    pub subtitles: Subtitle,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Body {
    pub media_id: String,
    pub media_category: String,
    pub subtitle_info: SubtitleInfo,
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
        let url = make_url(&self.twapi_options, Some(URL));
        let builder = client.post(&url).json(&self.body);
        authentication.execute(builder, "POST", &url, &[])
    }

    pub async fn execute(self, authentication: &impl Authentication) -> Result<Headers, Error> {
        execute_no_response(self.build(authentication)).await
    }
}
