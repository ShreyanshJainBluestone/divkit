// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTransform {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_x: Option<DivPivot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_y: Option<DivPivot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Expression<f64>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTransformBuilder {
    pub pivot_x: Option<DivPivot>,
    pub pivot_y: Option<DivPivot>,
    pub rotation: Option<Expression<f64>>,
}

impl DivTransform {
    pub fn builder() -> DivTransformBuilder {
        DivTransformBuilder::default()
    }
    pub fn default_pivot_x() -> DivPivot { serde_json::from_str(r#"{"type": "pivot-percentage","value":50}"#).unwrap() }
    pub fn default_pivot_y() -> DivPivot { serde_json::from_str(r#"{"type": "pivot-percentage","value":50}"#).unwrap() }
}

impl DivTransformBuilder {
    pub fn pivot_x(mut self, value: DivPivot) -> Self {
        self.pivot_x = Some(value);
        self
    }
    pub fn pivot_y(mut self, value: DivPivot) -> Self {
        self.pivot_y = Some(value);
        self
    }
    pub fn rotation(mut self, value: Expression<f64>) -> Self {
        self.rotation = Some(value);
        self
    }
    pub fn build(self) -> DivTransform {
        DivTransform {
            pivot_x: self.pivot_x.or_else(|| Some(DivTransform::default_pivot_x())),
            pivot_y: self.pivot_y.or_else(|| Some(DivTransform::default_pivot_y())),
            rotation: self.rotation,
        }
    }
}
