// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArrayVariable {
    pub name: String,
    #[serde(default = "ArrayVariable::default_type")]
    pub r#type: String,
    pub value: Expression<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default)]
pub struct ArrayVariableBuilder {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<Expression<Vec<serde_json::Value>>>,
}

impl ArrayVariable {
    pub fn builder() -> ArrayVariableBuilder {
        ArrayVariableBuilder::default()
    }
    pub fn default_type() -> String { "array".to_string() }
}

impl ArrayVariableBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<Vec<serde_json::Value>>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> ArrayVariable {
        ArrayVariable {
            name: self.name.expect("missing required field 'name'"),
            r#type: String::from("array"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
