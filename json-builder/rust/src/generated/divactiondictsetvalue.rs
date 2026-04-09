// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionDictSetValue {
    pub key: Expression<String>,
    #[serde(default = "DivActionDictSetValue::default_type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DivTypedValue>,
    pub variable_name: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionDictSetValueBuilder {
    pub key: Option<Expression<String>>,
    pub r#type: Option<String>,
    pub value: Option<DivTypedValue>,
    pub variable_name: Option<Expression<String>>,
}

impl DivActionDictSetValue {
    pub fn builder() -> DivActionDictSetValueBuilder {
        DivActionDictSetValueBuilder::default()
    }
    pub fn default_type() -> String { "dict_set_value".to_string() }
}

impl DivActionDictSetValueBuilder {
    pub fn key(mut self, value: Expression<String>) -> Self {
        self.key = Some(value);
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
    pub fn build(self) -> DivActionDictSetValue {
        DivActionDictSetValue {
            key: self.key.expect("missing required field 'key'"),
            r#type: String::from("dict_set_value"),
            value: self.value,
            variable_name: self.variable_name.expect("missing required field 'variable_name'"),
        }
    }
}
