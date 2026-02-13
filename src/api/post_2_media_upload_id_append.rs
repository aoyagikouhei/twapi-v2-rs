use std::io::Cursor;

use crate::{
    api::{Authentication, TwapiOptions, apply_options, make_url},
    error::Error,
    headers::Headers,
};
use reqwest::RequestBuilder;
use reqwest::multipart::{Form, Part};

const URL: &str = "/2/media/upload/:id/append";

#[derive(Debug, Clone, Default)]
pub struct FormData {
    pub segment_index: u64,
    pub cursor: Cursor<Vec<u8>>,
}

impl FormData {
    fn make_form(&self) -> Form {
        Form::new()
            .text("segment_index", self.segment_index.to_string())
            .part("media", Part::bytes(self.cursor.clone().into_inner()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    id: String,
    form: FormData,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(id: &str, form: FormData) -> Self {
        Self {
            id: id.to_string(),
            form,
            ..Default::default()
        }
    }

    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, &URL.replace(":id", &self.id));
        let builder = client.post(&url).multipart(self.form.make_form());
        authentication.execute(
            apply_options(builder, &self.twapi_options),
            "POST",
            &url,
            &[],
        )
    }

    pub async fn execute(&self, authentication: &impl Authentication) -> Result<Headers, Error> {
        let response = self.build(authentication).send().await?;
        let status_code = response.status();
        let header = response.headers();
        let headers = Headers::new(header);
        if status_code.is_success() {
            Ok(headers)
        } else {
            let body = response.text().await?;
            Err(Error::Other(body, Some(status_code)))
        }
    }
}
