// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivInputFilterRegex {
    pub pattern: Expression<String>,
    #[serde(default = "DivInputFilterRegex::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivInputFilterRegexBuilder {
    pub pattern: Option<Expression<String>>,
    pub r#type: Option<String>,
}

impl DivInputFilterRegex {
    pub fn builder() -> DivInputFilterRegexBuilder {
        DivInputFilterRegexBuilder::default()
    }
    pub fn default_type() -> String { "regex".to_string() }
}

impl DivInputFilterRegexBuilder {
    pub fn pattern(mut self, value: Expression<String>) -> Self {
        self.pattern = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivInputFilterRegex {
        DivInputFilterRegex {
            pattern: self.pattern.expect("missing required field 'pattern'"),
            r#type: String::from("regex"),
        }
    }
}
