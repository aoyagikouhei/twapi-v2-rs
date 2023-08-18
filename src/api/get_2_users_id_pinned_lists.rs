use itertools::Itertools;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::fields::{list_fields::ListFields, user_fields::UserFields};
use crate::responses::{memberships::Memberships, errors::Errors, includes::Includes, meta::Meta};
use reqwest::RequestBuilder;
use crate::{error::Error, rate_limit::RateLimit, api::{execute_twitter, Auth}};

const URL: &str = "https://api.twitter.com/2/users/:id/pinned_lists";



#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum Expansions {
    #[serde(rename = "owner_id")]
    OwnerId,
}

impl Expansions {
    pub fn all() -> HashSet<Self> {
        let mut result = HashSet::new();
        result.insert(Self::OwnerId);
        result
    }
}

impl std::fmt::Display for Expansions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OwnerId => write!(f, "owner_id"),
        }
    }
}

impl Default for Expansions {
    fn default() -> Self { Self::OwnerId }
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    id: String,
    expansions: Option<HashSet<Expansions>>,
    list_fields: Option<HashSet<ListFields>>,
    user_fields: Option<HashSet<UserFields>>,
}

impl Api {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            ..Default::default()
        }
    }
    
    pub fn all(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            expansions: Some(Expansions::all()),
            list_fields: Some(ListFields::all()),
            user_fields: Some(UserFields::all()),
        }
    }
    
    pub fn expansions(mut self, value: HashSet<Expansions>) -> Self {
        self.expansions = Some(value);
        self
    }
    
    pub fn list_fields(mut self, value: HashSet<ListFields>) -> Self {
        self.list_fields = Some(value);
        self
    }
    
    pub fn user_fields(mut self, value: HashSet<UserFields>) -> Self {
        self.user_fields = Some(value);
        self
    }

    pub fn build(self) -> RequestBuilder {
        let mut query_parameters = vec![];
        if let Some(expansions) = self.expansions {
            query_parameters.push(("expansions", expansions.iter().join(",")));
        }
        if let Some(list_fields) = self.list_fields {
            query_parameters.push(("list.fields", list_fields.iter().join(",")));
        }
        if let Some(user_fields) = self.user_fields {
            query_parameters.push(("user.fields", user_fields.iter().join(",")));
        }
        let client = reqwest::Client::new();
        client
            .get(URL.replace(":id", &self.id))
            .query(&query_parameters)
    }

    pub async fn execute(self, auth: &impl Auth) -> Result<(Response, Option<RateLimit>), Error> {
        execute_twitter(self.build(), auth).await
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub data: Option<Vec<Memberships>>, 
    pub errors: Option<Vec<Errors>>, 
    pub includes: Option<Includes>, 
    pub meta: Option<Meta>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.data.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.errors.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true) &&
        self.includes.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true) &&
        self.meta.as_ref().map(|it| it.is_empty_extra()).unwrap_or(true);
        if !res {
          println!("Response {:?}", self.extra);
        }
        res
    }
}
