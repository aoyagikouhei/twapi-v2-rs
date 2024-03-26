use serde::{Deserialize, Serialize};

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
