// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivDimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Expression<DivSizeUnit>>,
    pub value: Expression<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivDimensionBuilder {
    pub unit: Option<Expression<DivSizeUnit>>,
    pub value: Option<Expression<f64>>,
}

impl DivDimension {
    pub fn builder() -> DivDimensionBuilder {
        DivDimensionBuilder::default()
    }
    pub fn default_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""dp""#).unwrap()) }
}

impl DivDimensionBuilder {
    pub fn unit(mut self, value: Expression<DivSizeUnit>) -> Self {
        self.unit = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<f64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DivDimension {
        DivDimension {
            unit: self.unit.or_else(|| Some(DivDimension::default_unit())),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
