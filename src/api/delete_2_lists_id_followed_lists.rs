use super::{execute_twitter, TwitterResult};
use reqwest::RequestBuilder;

const URL: &str = "https://api.twitter.com/2/users/:id/followed_lists/:list_id";

#[derive(Debug, Clone, Default)]
pub struct Api {
    bearer_code: String,
    id: String,
    list_id: String,
}

impl Api {
    pub fn new(bearer_code: &str, id: &str, list_id: &str) -> Self {
        Self {
            bearer_code: bearer_code.to_owned(),
            id: id.to_owned(),
            list_id: list_id.to_owned(),
        }
    }

    pub fn build(self) -> RequestBuilder {
        let client = reqwest::Client::new();
        client
            .delete(
                URL.replace(":id", &self.id)
                    .replace(":list_id", &self.list_id),
            )
            .bearer_auth(self.bearer_code)
    }

    pub async fn execute(self) -> TwitterResult {
        execute_twitter(self.build()).await
    }
}