// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivInputValidatorExpression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty: Option<Expression<bool>>,
    pub condition: Expression<bool>,
    pub label_id: Expression<String>,
    #[serde(default = "DivInputValidatorExpression::default_type")]
    pub r#type: String,
    pub variable: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivInputValidatorExpressionBuilder {
    pub allow_empty: Option<Expression<bool>>,
    pub condition: Option<Expression<bool>>,
    pub label_id: Option<Expression<String>>,
    pub r#type: Option<String>,
    pub variable: Option<String>,
}

impl DivInputValidatorExpression {
    pub fn builder() -> DivInputValidatorExpressionBuilder {
        DivInputValidatorExpressionBuilder::default()
    }
    pub fn default_allow_empty() -> Expression<bool> { Expression::value(true) }
    pub fn default_type() -> String { "expression".to_string() }
}

impl DivInputValidatorExpressionBuilder {
    pub fn allow_empty(mut self, value: Expression<bool>) -> Self {
        self.allow_empty = Some(value);
        self
    }
    pub fn condition(mut self, value: Expression<bool>) -> Self {
        self.condition = Some(value);
        self
    }
    pub fn label_id(mut self, value: Expression<String>) -> Self {
        self.label_id = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn variable(mut self, value: String) -> Self {
        self.variable = Some(value);
        self
    }
    pub fn build(self) -> DivInputValidatorExpression {
        DivInputValidatorExpression {
            allow_empty: self.allow_empty.or_else(|| Some(DivInputValidatorExpression::default_allow_empty())),
            condition: self.condition.expect("missing required field 'condition'"),
            label_id: self.label_id.expect("missing required field 'label_id'"),
            r#type: String::from("expression"),
            variable: self.variable.expect("missing required field 'variable'"),
        }
    }
}
