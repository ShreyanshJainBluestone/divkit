// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ContentUrl {
    #[serde(default = "ContentUrl::default_type")]
    pub r#type: String,
    pub value: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ContentUrlBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<String>>,
}

impl ContentUrl {
    pub fn builder() -> ContentUrlBuilder {
        ContentUrlBuilder::default()
    }
    pub fn default_type() -> String { "url".to_string() }
}

impl ContentUrlBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<String>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> ContentUrl {
        ContentUrl {
            r#type: String::from("url"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
