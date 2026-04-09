// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DictValue {
    #[serde(default = "DictValue::default_type")]
    pub r#type: String,
    pub value: Expression<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default)]
pub struct DictValueBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<std::collections::HashMap<String, serde_json::Value>>>,
}

impl DictValue {
    pub fn builder() -> DictValueBuilder {
        DictValueBuilder::default()
    }
    pub fn default_type() -> String { "dict".to_string() }
}

impl DictValueBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<std::collections::HashMap<String, serde_json::Value>>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DictValue {
        DictValue {
            r#type: String::from("dict"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
