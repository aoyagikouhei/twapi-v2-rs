use crate::responses::errors::Errors;
use crate::responses::media_upload::MediaUpload;
use crate::{
    api::{apply_options, execute_twitter, make_url, Authentication, TwapiOptions},
    error::Error,
    headers::Headers,
};
use reqwest::multipart::Form;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/2/media/upload";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MediaCategory {
    AmplifyVideo,
    TweetGif,
    TweetImage,
    TweetVideo,
    DmVideo,
    DmImage,
    Subtitles,
    DmGif,
}

impl std::fmt::Display for MediaCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = match self {
            Self::AmplifyVideo => "amplify_video",
            Self::TweetGif => "tweet_gif",
            Self::TweetImage => "tweet_image",
            Self::TweetVideo => "tweet_video",
            Self::DmVideo => "dm_video",
            Self::DmImage => "dm_image",
            Self::Subtitles => "subtitles",
            Self::DmGif => "dm_gif",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FormData {
    pub total_bytes: u64,
    pub media_type: String,
    pub media_category: Option<MediaCategory>,
    pub additional_owners: Option<String>,
}

impl FormData {
    fn make_form(self) -> Form {
        let mut res = Form::new()
            .text("command", "INIT")
            .text("total_bytes", self.total_bytes.to_string())
            .text("media_type", self.media_type);
        if let Some(media_category) = self.media_category {
            res = res.text("media_category", media_category.to_string());
        }
        if let Some(additional_owners) = self.additional_owners {
            res = res.text("additional_owners", additional_owners);
        }
        res
    }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    form: FormData,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(form: FormData) -> Self {
        Self {
            form,
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
        let builder = client.post(&url).multipart(self.form.make_form());
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MediaUpload>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
