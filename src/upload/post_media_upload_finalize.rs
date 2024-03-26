use crate::{
    api::{execute_twitter, Authentication},
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
}

impl Api {
    pub fn new(data: Data) -> Self {
        Self { data }
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = make_url();
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
