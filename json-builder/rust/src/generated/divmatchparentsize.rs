// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivMatchParentSize {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<DivSizeUnitValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<DivSizeUnitValue>,
    #[serde(default = "DivMatchParentSize::default_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<Expression<f64>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivMatchParentSizeBuilder {
    pub max_size: Option<DivSizeUnitValue>,
    pub min_size: Option<DivSizeUnitValue>,
    pub r#type: Option<String>,
    pub weight: Option<Expression<f64>>,
}

impl DivMatchParentSize {
    pub fn builder() -> DivMatchParentSizeBuilder {
        DivMatchParentSizeBuilder::default()
    }
    pub fn default_type() -> String { "match_parent".to_string() }
}

impl DivMatchParentSizeBuilder {
    pub fn max_size(mut self, value: DivSizeUnitValue) -> Self {
        self.max_size = Some(value);
        self
    }
    pub fn min_size(mut self, value: DivSizeUnitValue) -> Self {
        self.min_size = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn weight(mut self, value: Expression<f64>) -> Self {
        self.weight = Some(value);
        self
    }
    pub fn build(self) -> DivMatchParentSize {
        DivMatchParentSize {
            max_size: self.max_size,
            min_size: self.min_size,
            r#type: String::from("match_parent"),
            weight: self.weight,
        }
    }
}
