use crate::{
    api::Authentication,
    error::Error,
    headers::Headers,
    upload::{execute_no_response, make_url_with_postfix},
};
use reqwest::RequestBuilder;
use serde::Serialize;

const URL: &str = "/1.1/media/metadata/create.json";

#[derive(Serialize, Debug, Clone, Default)]
pub struct AltText {
    pub text: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Body {
    pub media_id: String,
    pub alt_text: AltText,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    body: Body,
}

impl Api {
    pub fn new(body: Body) -> Self {
        Self { body }
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = make_url_with_postfix(URL);
        let builder = client.post(&url).json(&self.body);
        authentication.execute(builder, "POST", &url, &[])
    }

    pub async fn execute(self, authentication: &impl Authentication) -> Result<Headers, Error> {
        execute_no_response(self.build(authentication)).await
    }
}
