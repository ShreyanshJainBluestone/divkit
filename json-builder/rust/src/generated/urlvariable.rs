// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UrlVariable {
    pub name: String,
    #[serde(default = "UrlVariable::default_type")]
    pub r#type: String,
    pub value: Expression<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UrlVariableBuilder {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<Expression<String>>,
}

impl UrlVariable {
    pub fn builder() -> UrlVariableBuilder {
        UrlVariableBuilder::default()
    }
    pub fn default_type() -> String { "url".to_string() }
}

impl UrlVariableBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<String>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> UrlVariable {
        UrlVariable {
            name: self.name.expect("missing required field 'name'"),
            r#type: String::from("url"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
