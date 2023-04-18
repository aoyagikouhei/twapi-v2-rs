use super::{execute_twitter, TwitterResult};
use reqwest::RequestBuilder;

const URL: &str = "https://api.twitter.com/2/users/:id/bookmarks/:tweet_id";

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    id: String,
    tweet_id: String,
}

impl Api {
    pub fn new(bearer_code: &str, id: &str, tweet_id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            id: id.to_owned(),
            tweet_id: tweet_id.to_owned(),
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .delete(
                URL.replace(":id", &self.id)
                    .replace(":tweet_id", &self.tweet_id),
            )
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}
