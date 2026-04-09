// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArrayValue {
    #[serde(default = "ArrayValue::default_type")]
    pub r#type: String,
    pub value: Expression<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default)]
pub struct ArrayValueBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<Vec<serde_json::Value>>>,
}

impl ArrayValue {
    pub fn builder() -> ArrayValueBuilder {
        ArrayValueBuilder::default()
    }
    pub fn default_type() -> String { "array".to_string() }
}

impl ArrayValueBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<Vec<serde_json::Value>>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> ArrayValue {
        ArrayValue {
            r#type: String::from("array"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
