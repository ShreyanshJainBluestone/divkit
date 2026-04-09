// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IntegerVariable {
    pub name: String,
    #[serde(default = "IntegerVariable::default_type")]
    pub r#type: String,
    pub value: Expression<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct IntegerVariableBuilder {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<Expression<i64>>,
}

impl IntegerVariable {
    pub fn builder() -> IntegerVariableBuilder {
        IntegerVariableBuilder::default()
    }
    pub fn default_type() -> String { "integer".to_string() }
}

impl IntegerVariableBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<i64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> IntegerVariable {
        IntegerVariable {
            name: self.name.expect("missing required field 'name'"),
            r#type: String::from("integer"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
