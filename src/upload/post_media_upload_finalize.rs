use crate::{
    api::{execute_twitter, Authentication, TwapiOptions},
    error::Error,
    headers::Headers,
    upload::{make_url, response::Response},
};
use reqwest::{multipart::Form, RequestBuilder};

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub media_id: String,
}

impl Data {
    fn make_form(self) -> Form {
        Form::new()
            .text("command", "FINALIZE")
            .text("media_id", self.media_id)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    data: Data,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(data: Data) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, None);
        let builder = client.post(&url).multipart(self.data.make_form());
        authentication.execute(builder, "POST", &url, &[])
    }

    pub async fn execute(
        self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Headers), Error> {
        execute_twitter(self.build(authentication)).await
    }
}
