use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Stickers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_annotation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Stickers {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Stickers {:?}", self.extra);
        }
        res
    }
}
