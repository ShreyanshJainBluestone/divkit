// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ColorVariable {
    pub name: String,
    #[serde(default = "ColorVariable::default_type")]
    pub r#type: String,
    pub value: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ColorVariableBuilder {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<Expression<String>>,
}

impl ColorVariable {
    pub fn builder() -> ColorVariableBuilder {
        ColorVariableBuilder::default()
    }
    pub fn default_type() -> String { "color".to_string() }
}

impl ColorVariableBuilder {
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
    pub fn build(self) -> ColorVariable {
        ColorVariable {
            name: self.name.expect("missing required field 'name'"),
            r#type: String::from("color"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
