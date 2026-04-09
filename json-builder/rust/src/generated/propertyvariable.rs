// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PropertyVariable {
    pub get: Expression<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value_variable_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<DivAction>>,
    #[serde(default = "PropertyVariable::default_type")]
    pub r#type: String,
    pub value_type: DivEvaluableType,
}

#[derive(Debug, Clone, Default)]
pub struct PropertyVariableBuilder {
    pub get: Option<Expression<String>>,
    pub name: Option<String>,
    pub new_value_variable_name: Option<String>,
    pub set: Option<Vec<DivAction>>,
    pub r#type: Option<String>,
    pub value_type: Option<DivEvaluableType>,
}

impl PropertyVariable {
    pub fn builder() -> PropertyVariableBuilder {
        PropertyVariableBuilder::default()
    }
    pub fn default_new_value_variable_name() -> String { "new_value".to_string() }
    pub fn default_type() -> String { "property".to_string() }
}

impl PropertyVariableBuilder {
    pub fn get(mut self, value: Expression<String>) -> Self {
        self.get = Some(value);
        self
    }
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn new_value_variable_name(mut self, value: String) -> Self {
        self.new_value_variable_name = Some(value);
        self
    }
    pub fn set(mut self, value: Vec<DivAction>) -> Self {
        self.set = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value_type(mut self, value: DivEvaluableType) -> Self {
        self.value_type = Some(value);
        self
    }
    pub fn build(self) -> PropertyVariable {
        PropertyVariable {
            get: self.get.expect("missing required field 'get'"),
            name: self.name.expect("missing required field 'name'"),
            new_value_variable_name: self.new_value_variable_name.or_else(|| Some(PropertyVariable::default_new_value_variable_name())),
            set: self.set,
            r#type: String::from("property"),
            value_type: self.value_type.expect("missing required field 'value_type'"),
        }
    }
}
