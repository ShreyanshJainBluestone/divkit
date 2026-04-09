// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NumberVariable {
    pub name: String,
    #[serde(default = "NumberVariable::default_type")]
    pub r#type: String,
    pub value: Expression<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct NumberVariableBuilder {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<Expression<f64>>,
}

impl NumberVariable {
    pub fn builder() -> NumberVariableBuilder {
        NumberVariableBuilder::default()
    }
    pub fn default_type() -> String { "number".to_string() }
}

impl NumberVariableBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<f64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> NumberVariable {
        NumberVariable {
            name: self.name.expect("missing required field 'name'"),
            r#type: String::from("number"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
