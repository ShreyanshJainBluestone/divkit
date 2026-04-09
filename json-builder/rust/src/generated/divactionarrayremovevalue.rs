// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionArrayRemoveValue {
    pub index: Expression<i64>,
    #[serde(default = "DivActionArrayRemoveValue::default_type")]
    pub r#type: String,
    pub variable_name: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionArrayRemoveValueBuilder {
    pub index: Option<Expression<i64>>,
    pub r#type: Option<String>,
    pub variable_name: Option<Expression<String>>,
}

impl DivActionArrayRemoveValue {
    pub fn builder() -> DivActionArrayRemoveValueBuilder {
        DivActionArrayRemoveValueBuilder::default()
    }
    pub fn default_type() -> String { "array_remove_value".to_string() }
}

impl DivActionArrayRemoveValueBuilder {
    pub fn index(mut self, value: Expression<i64>) -> Self {
        self.index = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn variable_name(mut self, value: Expression<String>) -> Self {
        self.variable_name = Some(value);
        self
    }
    pub fn build(self) -> DivActionArrayRemoveValue {
        DivActionArrayRemoveValue {
            index: self.index.expect("missing required field 'index'"),
            r#type: String::from("array_remove_value"),
            variable_name: self.variable_name.expect("missing required field 'variable_name'"),
        }
    }
}
