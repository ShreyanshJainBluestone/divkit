// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivDefaultIndicatorItemPlacement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_between_centers: Option<DivFixedSize>,
    #[serde(default = "DivDefaultIndicatorItemPlacement::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivDefaultIndicatorItemPlacementBuilder {
    pub space_between_centers: Option<DivFixedSize>,
    pub r#type: Option<String>,
}

impl DivDefaultIndicatorItemPlacement {
    pub fn builder() -> DivDefaultIndicatorItemPlacementBuilder {
        DivDefaultIndicatorItemPlacementBuilder::default()
    }
    pub fn default_space_between_centers() -> DivFixedSize { serde_json::from_str(r#"{"type": "fixed","value":15}"#).unwrap() }
    pub fn default_type() -> String { "default".to_string() }
}

impl DivDefaultIndicatorItemPlacementBuilder {
    pub fn space_between_centers(mut self, value: DivFixedSize) -> Self {
        self.space_between_centers = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivDefaultIndicatorItemPlacement {
        DivDefaultIndicatorItemPlacement {
            space_between_centers: self.space_between_centers.or_else(|| Some(DivDefaultIndicatorItemPlacement::default_space_between_centers())),
            r#type: String::from("default"),
        }
    }
}
