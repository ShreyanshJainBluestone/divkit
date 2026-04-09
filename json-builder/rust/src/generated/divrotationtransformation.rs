// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivRotationTransformation {
    pub angle: Expression<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_x: Option<DivPivot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_y: Option<DivPivot>,
    #[serde(default = "DivRotationTransformation::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivRotationTransformationBuilder {
    pub angle: Option<Expression<f64>>,
    pub pivot_x: Option<DivPivot>,
    pub pivot_y: Option<DivPivot>,
    pub r#type: Option<String>,
}

impl DivRotationTransformation {
    pub fn builder() -> DivRotationTransformationBuilder {
        DivRotationTransformationBuilder::default()
    }
    pub fn default_pivot_x() -> DivPivot { serde_json::from_str(r#"{"type": "pivot-percentage","value":50}"#).unwrap() }
    pub fn default_pivot_y() -> DivPivot { serde_json::from_str(r#"{"type": "pivot-percentage","value":50}"#).unwrap() }
    pub fn default_type() -> String { "rotation".to_string() }
}

impl DivRotationTransformationBuilder {
    pub fn angle(mut self, value: Expression<f64>) -> Self {
        self.angle = Some(value);
        self
    }
    pub fn pivot_x(mut self, value: DivPivot) -> Self {
        self.pivot_x = Some(value);
        self
    }
    pub fn pivot_y(mut self, value: DivPivot) -> Self {
        self.pivot_y = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivRotationTransformation {
        DivRotationTransformation {
            angle: self.angle.expect("missing required field 'angle'"),
            pivot_x: self.pivot_x.or_else(|| Some(DivRotationTransformation::default_pivot_x())),
            pivot_y: self.pivot_y.or_else(|| Some(DivRotationTransformation::default_pivot_y())),
            r#type: String::from("rotation"),
        }
    }
}
