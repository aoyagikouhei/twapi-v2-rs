use crate::{
    api::{Authentication, TwapiOptions, execute_twitter},
    error::Error,
    headers::Headers,
    upload::{make_url, media_category::MediaCategory, response::Response},
};
use reqwest::{RequestBuilder, multipart::Form};

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
