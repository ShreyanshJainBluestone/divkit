// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTextRangeMaskSolid {
    pub color: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<Expression<bool>>,
    #[serde(default = "DivTextRangeMaskSolid::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivTextRangeMaskSolidBuilder {
    pub color: Option<Expression<String>>,
    pub is_enabled: Option<Expression<bool>>,
    pub r#type: Option<String>,
}

impl DivTextRangeMaskSolid {
    pub fn builder() -> DivTextRangeMaskSolidBuilder {
        DivTextRangeMaskSolidBuilder::default()
    }
    pub fn default_is_enabled() -> Expression<bool> { Expression::value(true) }
    pub fn default_type() -> String { "solid".to_string() }
}

impl DivTextRangeMaskSolidBuilder {
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn is_enabled(mut self, value: Expression<bool>) -> Self {
        self.is_enabled = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivTextRangeMaskSolid {
        DivTextRangeMaskSolid {
            color: self.color.expect("missing required field 'color'"),
            is_enabled: self.is_enabled.or_else(|| Some(DivTextRangeMaskSolid::default_is_enabled())),
            r#type: String::from("solid"),
        }
    }
}
