// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTextRangeMaskBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<Expression<bool>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTextRangeMaskBaseBuilder {
    pub is_enabled: Option<Expression<bool>>,
}

impl DivTextRangeMaskBase {
    pub fn builder() -> DivTextRangeMaskBaseBuilder {
        DivTextRangeMaskBaseBuilder::default()
    }
    pub fn default_is_enabled() -> Expression<bool> { Expression::value(true) }
}

impl DivTextRangeMaskBaseBuilder {
    pub fn is_enabled(mut self, value: Expression<bool>) -> Self {
        self.is_enabled = Some(value);
        self
    }
    pub fn build(self) -> DivTextRangeMaskBase {
        DivTextRangeMaskBase {
            is_enabled: self.is_enabled.or_else(|| Some(DivTextRangeMaskBase::default_is_enabled())),
        }
    }
}
