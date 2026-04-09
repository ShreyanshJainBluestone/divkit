// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IntegerValue {
    #[serde(default = "IntegerValue::default_type")]
    pub r#type: String,
    pub value: Expression<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct IntegerValueBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<i64>>,
}

impl IntegerValue {
    pub fn builder() -> IntegerValueBuilder {
        IntegerValueBuilder::default()
    }
    pub fn default_type() -> String { "integer".to_string() }
}

impl IntegerValueBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<i64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> IntegerValue {
        IntegerValue {
            r#type: String::from("integer"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
