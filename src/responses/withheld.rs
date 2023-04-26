use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Withheld {
    pub copyright: Option<bool>,
    pub country_codes: Option<Vec<String>>,
    pub scope: Option<Scope>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Withheld {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Withheld {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Scope {
    #[serde(rename = "tweet")]
    Tweet,
    #[serde(rename = "user")]
    User,
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tweet => write!(f, "tweet"),
            Self::User => write!(f, "user"),
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::Tweet
    }
}
