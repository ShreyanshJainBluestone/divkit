// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ContentText {
    #[serde(default = "ContentText::default_type")]
    pub r#type: String,
    pub value: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ContentTextBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<String>>,
}

impl ContentText {
    pub fn builder() -> ContentTextBuilder {
        ContentTextBuilder::default()
    }
    pub fn default_type() -> String { "text".to_string() }
}

impl ContentTextBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<String>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> ContentText {
        ContentText {
            r#type: String::from("text"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
