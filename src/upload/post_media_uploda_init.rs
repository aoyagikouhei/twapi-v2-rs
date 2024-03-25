use crate::{
    api::{execute_twitter, Authentication},
    error::Error,
    headers::Headers,
    upload::{make_url, MediaCategory, Response},
};
use reqwest::{multipart::Form, RequestBuilder};

const URL: &str = "/1.1/media/upload.json";

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub total_bytes: u64,
    pub media_type: String,
    pub media_category: Option<MediaCategory>,
    pub additional_owners: Option<String>,
}

impl Data {
    fn make_form(self) -> Form {
        let mut res = Form::new()
            .text("command", "INIT")
            .text("total_bytes", self.total_bytes.to_string())
            .text("media_type", self.media_type);
        if let Some(media_category) = self.media_category {
            res = res.text("media_category", media_category.to_string());
        }
        if let Some(additional_owners) = self.additional_owners {
            res = res.text("additional_owners", additional_owners);
        }
        res
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
        let url = make_url(URL);
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
