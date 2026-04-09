// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivShapeDrawable {
    pub color: Expression<String>,
    pub shape: DivShape,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<DivStroke>,
    #[serde(default = "DivShapeDrawable::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivShapeDrawableBuilder {
    pub color: Option<Expression<String>>,
    pub shape: Option<DivShape>,
    pub stroke: Option<DivStroke>,
    pub r#type: Option<String>,
}

impl DivShapeDrawable {
    pub fn builder() -> DivShapeDrawableBuilder {
        DivShapeDrawableBuilder::default()
    }
    pub fn default_type() -> String { "shape_drawable".to_string() }
}

impl DivShapeDrawableBuilder {
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn shape(mut self, value: DivShape) -> Self {
        self.shape = Some(value);
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
    pub fn build(self) -> DivShapeDrawable {
        DivShapeDrawable {
            color: self.color.expect("missing required field 'color'"),
            shape: self.shape.expect("missing required field 'shape'"),
            stroke: self.stroke,
            r#type: String::from("shape_drawable"),
        }
    }
}
