// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivCircleShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<DivFixedSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<DivStroke>,
    #[serde(default = "DivCircleShape::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivCircleShapeBuilder {
    pub background_color: Option<Expression<String>>,
    pub radius: Option<DivFixedSize>,
    pub stroke: Option<DivStroke>,
    pub r#type: Option<String>,
}

impl DivCircleShape {
    pub fn builder() -> DivCircleShapeBuilder {
        DivCircleShapeBuilder::default()
    }
    pub fn default_radius() -> DivFixedSize { serde_json::from_str(r#"{"type":"fixed","value":10}"#).unwrap() }
    pub fn default_type() -> String { "circle".to_string() }
}

impl DivCircleShapeBuilder {
    pub fn background_color(mut self, value: Expression<String>) -> Self {
        self.background_color = Some(value);
        self
    }
    pub fn radius(mut self, value: DivFixedSize) -> Self {
        self.radius = Some(value);
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
    pub fn build(self) -> DivCircleShape {
        DivCircleShape {
            background_color: self.background_color,
            radius: self.radius.or_else(|| Some(DivCircleShape::default_radius())),
            stroke: self.stroke,
            r#type: String::from("circle"),
        }
    }
}
