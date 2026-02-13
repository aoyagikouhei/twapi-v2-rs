use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ReferencedTweets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ReferencedTweets {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
          println!("ReferencedTweets {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Type {
    #[serde(rename = "retweeted")]
    Retweeted,
    #[serde(rename = "quoted")]
    Quoted,
    #[serde(rename = "replied_to")]
    RepliedTo,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Retweeted => write!(f, "retweeted"),
            Self::Quoted => write!(f, "quoted"),
            Self::RepliedTo => write!(f, "replied_to"),
        }
    }
}

impl Default for Type {
    fn default() -> Self { Self::Retweeted }
}
