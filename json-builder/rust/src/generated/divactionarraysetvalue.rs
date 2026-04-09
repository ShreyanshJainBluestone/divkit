// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionArraySetValue {
    pub index: Expression<i64>,
    #[serde(default = "DivActionArraySetValue::default_type")]
    pub r#type: String,
    pub value: DivTypedValue,
    pub variable_name: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionArraySetValueBuilder {
    pub index: Option<Expression<i64>>,
    pub r#type: Option<String>,
    pub value: Option<DivTypedValue>,
    pub variable_name: Option<Expression<String>>,
}

impl DivActionArraySetValue {
    pub fn builder() -> DivActionArraySetValueBuilder {
        DivActionArraySetValueBuilder::default()
    }
    pub fn default_type() -> String { "array_set_value".to_string() }
}

impl DivActionArraySetValueBuilder {
    pub fn index(mut self, value: Expression<i64>) -> Self {
        self.index = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: DivTypedValue) -> Self {
        self.value = Some(value);
        self
    }
    pub fn variable_name(mut self, value: Expression<String>) -> Self {
        self.variable_name = Some(value);
        self
    }
    pub fn build(self) -> DivActionArraySetValue {
        DivActionArraySetValue {
            index: self.index.expect("missing required field 'index'"),
            r#type: String::from("array_set_value"),
            value: self.value.expect("missing required field 'value'"),
            variable_name: self.variable_name.expect("missing required field 'variable_name'"),
        }
    }
}
