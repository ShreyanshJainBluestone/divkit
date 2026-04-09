// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ColorValue {
    #[serde(default = "ColorValue::default_type")]
    pub r#type: String,
    pub value: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ColorValueBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<String>>,
}

impl ColorValue {
    pub fn builder() -> ColorValueBuilder {
        ColorValueBuilder::default()
    }
    pub fn default_type() -> String { "color".to_string() }
}

impl ColorValueBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<String>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> ColorValue {
        ColorValue {
            r#type: String::from("color"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
