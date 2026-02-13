use crate::responses::{usage::Usage};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DailyClientAppUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_app_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_result_count: Option<i64>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<Usage>>, 
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl DailyClientAppUsage {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty() &&
        self.usage.as_ref().map(|it| it.iter().all(|item| item.is_empty_extra())).unwrap_or(true);
        if !res {
          println!("DailyClientAppUsage {:?}", self.extra);
        }
        res
    }
}
