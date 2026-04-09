// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFixedCount {
    #[serde(default = "DivFixedCount::default_type")]
    pub r#type: String,
    pub value: Expression<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivFixedCountBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<i64>>,
}

impl DivFixedCount {
    pub fn builder() -> DivFixedCountBuilder {
        DivFixedCountBuilder::default()
    }
    pub fn default_type() -> String { "fixed".to_string() }
}

impl DivFixedCountBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<i64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DivFixedCount {
        DivFixedCount {
            r#type: String::from("fixed"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
