use serde::{Serialize, Deserialize};
use crate::responses::{compliance::Compliance};
use reqwest::RequestBuilder;
use crate::{error::Error, headers::Headers, api::{apply_options, execute_twitter, Authentication, make_url, TwapiOptions}};

const URL: &str = "/2/users/compliance/stream";





#[derive(Debug, Clone, Default)]
pub struct Api {
    partition: usize,
    backfill_minutes: Option<usize>,
    twapi_options: Option<TwapiOptions>,
}

impl Api {
    pub fn new(partition: usize) -> Self {
        Self {
            partition,
            ..Default::default()
        }
    }
    
    pub fn backfill_minutes(mut self, value: usize) -> Self {
        self.backfill_minutes = Some(value);
        self
    }
    
    
    pub fn twapi_options(mut self, value: TwapiOptions) -> Self {
        self.twapi_options = Some(value);
        self
    }

    pub fn build(&self, authentication: &impl Authentication) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("partition", self.partition.to_string()));
        if let Some(backfill_minutes) = self.backfill_minutes.as_ref() {
            query_parameters.push(("backfill_minutes", backfill_minutes.to_string()));
        }
        let client = reqwest::Client::new();
        let url = make_url(&self.twapi_options, URL);
        let builder = client
            .get(&url)
            .query(&query_parameters)
        ;
        authentication.execute(apply_options(builder, &self.twapi_options), "GET", &url, &query_parameters.iter().map(|it| (it.0, it.1.as_str())).collect::<Vec<_>>())
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
    pub user_delete: Option<Compliance>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_undelete: Option<Compliance>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_withheld: Option<Compliance>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_protect: Option<Compliance>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_unprotect: Option<Compliance>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_suspend: Option<Compliance>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_unsuspend: Option<Compliance>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrub_geo: Option<Compliance>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_modification: Option<Compliance>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.user_delete.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.user_undelete.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.user_withheld.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.user_protect.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.user_unprotect.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.user_suspend.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.user_unsuspend.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.scrub_geo.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.user_profile_modification.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Data {:?}", self.extra);
        }
        res
    }
}
