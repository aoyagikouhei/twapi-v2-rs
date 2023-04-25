use crate::responses::{media::Media, places::Places, polls::Polls, tweets::Tweets, users::Users};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Includes {
    pub media: Option<Vec<Media>>,
    pub places: Option<Vec<Places>>,
    pub polls: Option<Vec<Polls>>,
    pub tweets: Option<Vec<Tweets>>,
    pub users: Option<Vec<Users>>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
