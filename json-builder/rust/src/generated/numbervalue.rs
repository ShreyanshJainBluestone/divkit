// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NumberValue {
    #[serde(default = "NumberValue::default_type")]
    pub r#type: String,
    pub value: Expression<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct NumberValueBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<f64>>,
}

impl NumberValue {
    pub fn builder() -> NumberValueBuilder {
        NumberValueBuilder::default()
    }
    pub fn default_type() -> String { "number".to_string() }
}

impl NumberValueBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<f64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> NumberValue {
        NumberValue {
            r#type: String::from("number"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
