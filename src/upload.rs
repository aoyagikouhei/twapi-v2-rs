use serde::{Deserialize, Serialize};

pub mod post_media_uploda_init;

const ENV_KEY: &str = "TWAPI_V2_MEDIA_API_PREFIX_API";
const PREFIX_URL_MEDIA: &str = "https://upload.twitter.com";

pub fn clear_prefix_url() {
    std::env::set_var(ENV_KEY, PREFIX_URL_MEDIA);
}

pub fn setup_prefix_url(url: &str) {
    std::env::set_var(ENV_KEY, url);
}

pub(crate) fn make_url<S: AsRef<str>>(post_url: S) -> String {
    let prefix_url = std::env::var(ENV_KEY).unwrap_or(PREFIX_URL_MEDIA.to_owned());
    format!("{}{}", prefix_url, post_url.as_ref())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MediaCategory {
    AmplifyVideo,
    TweetGif,
    TweetImage,
    TweetVideo,
}

impl std::fmt::Display for MediaCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = match self {
            Self::AmplifyVideo => "amplify_video",
            Self::TweetGif => "tweet_gif",
            Self::TweetImage => "tweet_image",
            Self::TweetVideo => "tweet_video",
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Image {
    pub image_type: String,
    pub w: u64,
    pub h: u64,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Video {
    pub video_type: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProcessingInfo {
    pub state: String,
    pub check_after_secs: Option<u64>,
    pub progress_percent: Option<u64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub media_id: u64,
    pub media_id_string: String,
    pub expires_after_secs: u64,
    pub media_key: Option<String>,
    pub size: Option<u64>,
    pub image: Option<Image>,
    pub video: Option<Video>,
    pub processing_info: Option<ProcessingInfo>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
