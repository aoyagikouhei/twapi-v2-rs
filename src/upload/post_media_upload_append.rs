use std::io::Cursor;

use crate::{api::Authentication, error::Error, headers::Headers, upload::make_url};
use reqwest::{
    multipart::{Form, Part},
    RequestBuilder,
};

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub media_id: String,
    pub segment_index: u64,
    pub cursor: Cursor<Vec<u8>>,
}

impl Data {
    fn make_form(self) -> Form {
        Form::new()
            .text("command", "APPEND")
            .text("media_id", self.media_id)
            .text("segment_index", self.segment_index.to_string())
            .part("media", Part::bytes(self.cursor.into_inner()))
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

    pub async fn execute(self, authentication: &impl Authentication) -> Result<Headers, Error> {
        let response = self.build(authentication).send().await?;
        let status_code = response.status();
        let header = response.headers();
        let headers = Headers::new(header);
        if status_code.is_success() {
            Ok(headers)
        } else {
            Err(Error::Other("append error".to_owned(), Some(status_code)))
        }
    }
}
