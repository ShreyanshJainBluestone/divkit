// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivInputFilterExpression {
    pub condition: Expression<bool>,
    #[serde(default = "DivInputFilterExpression::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivInputFilterExpressionBuilder {
    pub condition: Option<Expression<bool>>,
    pub r#type: Option<String>,
}

impl DivInputFilterExpression {
    pub fn builder() -> DivInputFilterExpressionBuilder {
        DivInputFilterExpressionBuilder::default()
    }
    pub fn default_type() -> String { "expression".to_string() }
}

impl DivInputFilterExpressionBuilder {
    pub fn condition(mut self, value: Expression<bool>) -> Self {
        self.condition = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivInputFilterExpression {
        DivInputFilterExpression {
            condition: self.condition.expect("missing required field 'condition'"),
            r#type: String::from("expression"),
        }
    }
}
