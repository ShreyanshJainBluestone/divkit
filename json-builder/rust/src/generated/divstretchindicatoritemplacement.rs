// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivStretchIndicatorItemPlacement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_spacing: Option<DivFixedSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_visible_items: Option<Expression<i64>>,
    #[serde(default = "DivStretchIndicatorItemPlacement::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivStretchIndicatorItemPlacementBuilder {
    pub item_spacing: Option<DivFixedSize>,
    pub max_visible_items: Option<Expression<i64>>,
    pub r#type: Option<String>,
}

impl DivStretchIndicatorItemPlacement {
    pub fn builder() -> DivStretchIndicatorItemPlacementBuilder {
        DivStretchIndicatorItemPlacementBuilder::default()
    }
    pub fn default_item_spacing() -> DivFixedSize { serde_json::from_str(r#"{"type": "fixed","value":5}"#).unwrap() }
    pub fn default_max_visible_items() -> Expression<i64> { Expression::value(10i64) }
    pub fn default_type() -> String { "stretch".to_string() }
}

impl DivStretchIndicatorItemPlacementBuilder {
    pub fn item_spacing(mut self, value: DivFixedSize) -> Self {
        self.item_spacing = Some(value);
        self
    }
    pub fn max_visible_items(mut self, value: Expression<i64>) -> Self {
        self.max_visible_items = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivStretchIndicatorItemPlacement {
        DivStretchIndicatorItemPlacement {
            item_spacing: self.item_spacing.or_else(|| Some(DivStretchIndicatorItemPlacement::default_item_spacing())),
            max_visible_items: self.max_visible_items.or_else(|| Some(DivStretchIndicatorItemPlacement::default_max_visible_items())),
            r#type: String::from("stretch"),
        }
    }
}
