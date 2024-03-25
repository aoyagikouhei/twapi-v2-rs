use crate::{
    api::{execute_twitter, Authentication},
    error::Error,
    headers::Headers,
    upload::make_url,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

const URL: &str = "/1.1/media/upload.json";

#[derive(Debug, Clone, Default)]
pub struct Form {
    total_bytes: u64,
    media_type: String,
    media_category: Option<String>,
    additional_owners: Option<String>,
}

impl Form {
    fn make_form(self) -> Vec<(&'static str, String)> {
        let mut res = vec![
            ("command", "INIT".to_owned()),
            ("total_bytes", self.total_bytes.to_string()),
            ("media_type", self.media_type),
        ];
        if let Some(media_category) = self.media_category {
            res.push(("media_category", media_category));
        }
        if let Some(additional_owners) = self.additional_owners {
            res.push(("additional_owners", additional_owners));
        }
        res
    }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    form: Form,
}

impl Api {
    pub fn new(form: Form) -> Self {
        Self { form }
    }

    pub fn build(self, authentication: &impl Authentication) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = make_url(URL);
        let builder = client.post(&url).form(&self.form.make_form());
        authentication.execute(builder, "POST", &url, &[])
    }

    pub async fn execute(
        self,
        authentication: &impl Authentication,
    ) -> Result<(Response, Headers), Error> {
        execute_twitter(self.build(authentication)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Image {
    pub image_type: String,
    pub w: u64,
    pub h: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub media_id: u64,
    pub media_id_string: String,
    pub size: u64,
    pub image: Option<Image>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}
