// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivWrapContentSize {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constrained: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<DivSizeUnitValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<DivSizeUnitValue>,
    #[serde(default = "DivWrapContentSize::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivWrapContentSizeBuilder {
    pub constrained: Option<Expression<bool>>,
    pub max_size: Option<DivSizeUnitValue>,
    pub min_size: Option<DivSizeUnitValue>,
    pub r#type: Option<String>,
}

impl DivWrapContentSize {
    pub fn builder() -> DivWrapContentSizeBuilder {
        DivWrapContentSizeBuilder::default()
    }
    pub fn default_type() -> String { "wrap_content".to_string() }
}

impl DivWrapContentSizeBuilder {
    pub fn constrained(mut self, value: Expression<bool>) -> Self {
        self.constrained = Some(value);
        self
    }
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
    pub fn build(self) -> DivWrapContentSize {
        DivWrapContentSize {
            constrained: self.constrained,
            max_size: self.max_size,
            min_size: self.min_size,
            r#type: String::from("wrap_content"),
        }
    }
}
