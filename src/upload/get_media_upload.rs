use crate::{
    api::{execute_twitter, Authentication},
    error::Error,
    headers::Headers,
    upload::{make_url, response::Response},
};
use reqwest::RequestBuilder;

#[derive(Debug, Clone, Default)]
pub struct Api {
    media_id: String,
}

impl Api {
    pub fn new(media_id: String) -> Self {
        Self { media_id }
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = make_url();
        let query = [("media_id", self.media_id.as_str()), ("command", "STATUS")];
        let builder = client.get(&url).query(&query);
        authentication.execute(builder, "GET", &url, &query)
    }

    pub async fn execute(
        self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Headers), Error> {
        execute_twitter(self.build(authentication)).await
    }
}
