use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DeletedSubtitles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_subtitles: Option<Vec<String>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_category: Option<MediaCategory>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl DeletedSubtitles {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("DeletedSubtitles {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MediaCategory {
    #[serde(rename = "amplify_video")]
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

impl Default for MediaCategory {
    fn default() -> Self { Self::AmplifyVideo }
}
