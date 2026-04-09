// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivSizeUnitValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Expression<DivSizeUnit>>,
    pub value: Expression<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivSizeUnitValueBuilder {
    pub unit: Option<Expression<DivSizeUnit>>,
    pub value: Option<Expression<i64>>,
}

impl DivSizeUnitValue {
    pub fn builder() -> DivSizeUnitValueBuilder {
        DivSizeUnitValueBuilder::default()
    }
    pub fn default_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""dp""#).unwrap()) }
}

impl DivSizeUnitValueBuilder {
    pub fn unit(mut self, value: Expression<DivSizeUnit>) -> Self {
        self.unit = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<i64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DivSizeUnitValue {
        DivSizeUnitValue {
            unit: self.unit.or_else(|| Some(DivSizeUnitValue::default_unit())),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
