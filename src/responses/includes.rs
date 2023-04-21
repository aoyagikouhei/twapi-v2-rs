use crate::responses::{media::Media, tweets::Tweets, users::Users};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Includes {
    pub media: Option<Vec<Media>>,
    pub tweets: Option<Vec<Tweets>>,
    pub users: Option<Vec<Users>>,
}
