// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DictVariable {
    pub name: String,
    #[serde(default = "DictVariable::default_type")]
    pub r#type: String,
    pub value: Expression<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default)]
pub struct DictVariableBuilder {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
}

impl DictVariable {
    pub fn builder() -> DictVariableBuilder {
        DictVariableBuilder::default()
    }
    pub fn default_type() -> String { "dict".to_string() }
}

impl DictVariableBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<std::collections::HashMap<String, serde_json::Value>>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DictVariable {
        DictVariable {
            name: self.name.expect("missing required field 'name'"),
            r#type: String::from("dict"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
