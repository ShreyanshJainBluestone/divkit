// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFixedSize {
    #[serde(default = "DivFixedSize::default_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Expression<DivSizeUnit>>,
    pub value: Expression<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivFixedSizeBuilder {
    pub r#type: Option<String>,
    pub unit: Option<Expression<DivSizeUnit>>,
    pub value: Option<Expression<i64>>,
}

impl DivFixedSize {
    pub fn builder() -> DivFixedSizeBuilder {
        DivFixedSizeBuilder::default()
    }
    pub fn default_type() -> String { "fixed".to_string() }
    pub fn default_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""dp""#).unwrap()) }
}

impl DivFixedSizeBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn unit(mut self, value: Expression<DivSizeUnit>) -> Self {
        self.unit = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<i64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DivFixedSize {
        DivFixedSize {
            r#type: String::from("fixed"),
            unit: self.unit.or_else(|| Some(DivFixedSize::default_unit())),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
