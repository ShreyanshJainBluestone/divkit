// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StringValue {
    #[serde(default = "StringValue::default_type")]
    pub r#type: String,
    pub value: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct StringValueBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<String>>,
}

impl StringValue {
    pub fn builder() -> StringValueBuilder {
        StringValueBuilder::default()
    }
    pub fn default_type() -> String { "string".to_string() }
}

impl StringValueBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<String>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> StringValue {
        StringValue {
            r#type: String::from("string"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
