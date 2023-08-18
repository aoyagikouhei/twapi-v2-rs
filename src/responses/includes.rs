use crate::responses::{media::Media, places::Places, polls::Polls, tweets::Tweets, users::Users};
use serde::{Serialize, Deserialize};

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

impl Includes {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.media.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.places.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.polls.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.tweets.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.users.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true);
        if !res {
          println!("Includes {:?}", self.extra);
        }
        res
    }
}
