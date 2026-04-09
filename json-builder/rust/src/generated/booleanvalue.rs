// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BooleanValue {
    #[serde(default = "BooleanValue::default_type")]
    pub r#type: String,
    pub value: Expression<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct BooleanValueBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<bool>>,
}

impl BooleanValue {
    pub fn builder() -> BooleanValueBuilder {
        BooleanValueBuilder::default()
    }
    pub fn default_type() -> String { "boolean".to_string() }
}

impl BooleanValueBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<bool>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> BooleanValue {
        BooleanValue {
            r#type: String::from("boolean"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
