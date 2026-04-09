// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivInputValidatorBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_id: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivInputValidatorBaseBuilder {
    pub allow_empty: Option<Expression<bool>>,
    pub label_id: Option<Expression<String>>,
    pub variable: Option<String>,
}

impl DivInputValidatorBase {
    pub fn builder() -> DivInputValidatorBaseBuilder {
        DivInputValidatorBaseBuilder::default()
    }
    pub fn default_allow_empty() -> Expression<bool> { Expression::value(true) }
}

impl DivInputValidatorBaseBuilder {
    pub fn allow_empty(mut self, value: Expression<bool>) -> Self {
        self.allow_empty = Some(value);
        self
    }
    pub fn label_id(mut self, value: Expression<String>) -> Self {
        self.label_id = Some(value);
        self
    }
    pub fn variable(mut self, value: String) -> Self {
        self.variable = Some(value);
        self
    }
    pub fn build(self) -> DivInputValidatorBase {
        DivInputValidatorBase {
            allow_empty: self.allow_empty.or_else(|| Some(DivInputValidatorBase::default_allow_empty())),
            label_id: self.label_id,
            variable: self.variable,
        }
    }
}
