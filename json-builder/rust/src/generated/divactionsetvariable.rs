// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionSetVariable {
    #[serde(default = "DivActionSetVariable::default_type")]
    pub r#type: String,
    pub value: DivTypedValue,
    pub variable_name: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionSetVariableBuilder {
    pub r#type: Option<String>,
    pub value: Option<DivTypedValue>,
    pub variable_name: Option<Expression<String>>,
}

impl DivActionSetVariable {
    pub fn builder() -> DivActionSetVariableBuilder {
        DivActionSetVariableBuilder::default()
    }
    pub fn default_type() -> String { "set_variable".to_string() }
}

impl DivActionSetVariableBuilder {
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
    pub fn build(self) -> DivActionSetVariable {
        DivActionSetVariable {
            r#type: String::from("set_variable"),
            value: self.value.expect("missing required field 'value'"),
            variable_name: self.variable_name.expect("missing required field 'variable_name'"),
        }
    }
}
