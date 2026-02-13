use serde::{Serialize, Deserialize};
use crate::responses::{associated_metadata::AssociatedMetadata};
use reqwest::RequestBuilder;
use crate::{error::Error, headers::Headers, api::{apply_options, execute_twitter, Authentication, make_url, TwapiOptions}};

const URL: &str = "/2/media/metadata/create";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AllowDownloadStatus {
    pub allow_download: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AltText {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FoundMediaOrigin {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Sticker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_annotation_id: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_annotation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_a: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_b: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_c: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_d: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_tx: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_ty: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StickerInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<Sticker>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UploadSource {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_download_status: Option<AllowDownloadStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<AltText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found_media_origin: Option<FoundMediaOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_info: Option<StickerInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_source: Option<UploadSource>,
}



#[derive(Debug, Clone, Default)]
pub struct Api {
    body: Body,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(body: Body) -> Self {
        Self {
            body,
            ..Default::default()
        }
    }
    
    
    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, URL);
        let builder = client
            .post(&url)
            .json(&self.body)
        ;
        authentication.execute(apply_options(builder, &self.twapi_options), "POST", &url, &[])
    }

    pub async fn execute(&self, authentication: &impl Authentication) -> Result<(Response, Headers), Error> {
        execute_twitter(|| self.build(authentication)).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_metadata: Option<AssociatedMetadata>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.associated_metadata.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Data {:?}", self.extra);
        }
        res
    }
}
