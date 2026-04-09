// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StringVariable {
    pub name: String,
    #[serde(default = "StringVariable::default_type")]
    pub r#type: String,
    pub value: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct StringVariableBuilder {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<Expression<String>>,
}

impl StringVariable {
    pub fn builder() -> StringVariableBuilder {
        StringVariableBuilder::default()
    }
    pub fn default_type() -> String { "string".to_string() }
}

impl StringVariableBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<String>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> StringVariable {
        StringVariable {
            name: self.name.expect("missing required field 'name'"),
            r#type: String::from("string"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
