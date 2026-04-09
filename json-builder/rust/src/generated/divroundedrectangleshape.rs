// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivRoundedRectangleShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<DivFixedSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_height: Option<DivFixedSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_width: Option<DivFixedSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<DivStroke>,
    #[serde(default = "DivRoundedRectangleShape::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivRoundedRectangleShapeBuilder {
    pub background_color: Option<Expression<String>>,
    pub corner_radius: Option<DivFixedSize>,
    pub item_height: Option<DivFixedSize>,
    pub item_width: Option<DivFixedSize>,
    pub stroke: Option<DivStroke>,
    pub r#type: Option<String>,
}

impl DivRoundedRectangleShape {
    pub fn builder() -> DivRoundedRectangleShapeBuilder {
        DivRoundedRectangleShapeBuilder::default()
    }
    pub fn default_corner_radius() -> DivFixedSize { serde_json::from_str(r#"{"type":"fixed","value":5}"#).unwrap() }
    pub fn default_item_height() -> DivFixedSize { serde_json::from_str(r#"{"type":"fixed","value":10}"#).unwrap() }
    pub fn default_item_width() -> DivFixedSize { serde_json::from_str(r#"{"type":"fixed","value":10}"#).unwrap() }
    pub fn default_type() -> String { "rounded_rectangle".to_string() }
}

impl DivRoundedRectangleShapeBuilder {
    pub fn background_color(mut self, value: Expression<String>) -> Self {
        self.background_color = Some(value);
        self
    }
    pub fn corner_radius(mut self, value: DivFixedSize) -> Self {
        self.corner_radius = Some(value);
        self
    }
    pub fn item_height(mut self, value: DivFixedSize) -> Self {
        self.item_height = Some(value);
        self
    }
    pub fn item_width(mut self, value: DivFixedSize) -> Self {
        self.item_width = Some(value);
        self
    }
    pub fn stroke(mut self, value: DivStroke) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivRoundedRectangleShape {
        DivRoundedRectangleShape {
            background_color: self.background_color,
            corner_radius: self.corner_radius.or_else(|| Some(DivRoundedRectangleShape::default_corner_radius())),
            item_height: self.item_height.or_else(|| Some(DivRoundedRectangleShape::default_item_height())),
            item_width: self.item_width.or_else(|| Some(DivRoundedRectangleShape::default_item_width())),
            stroke: self.stroke,
            r#type: String::from("rounded_rectangle"),
        }
    }
}
