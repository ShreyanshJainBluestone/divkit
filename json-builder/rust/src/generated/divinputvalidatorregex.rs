// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivInputValidatorRegex {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty: Option<Expression<bool>>,
    pub label_id: Expression<String>,
    pub pattern: Expression<String>,
    #[serde(default = "DivInputValidatorRegex::default_type")]
    pub r#type: String,
    pub variable: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivInputValidatorRegexBuilder {
    pub allow_empty: Option<Expression<bool>>,
    pub label_id: Option<Expression<String>>,
    pub pattern: Option<Expression<String>>,
    pub r#type: Option<String>,
    pub variable: Option<String>,
}

impl DivInputValidatorRegex {
    pub fn builder() -> DivInputValidatorRegexBuilder {
        DivInputValidatorRegexBuilder::default()
    }
    pub fn default_allow_empty() -> Expression<bool> { Expression::value(true) }
    pub fn default_type() -> String { "regex".to_string() }
}

impl DivInputValidatorRegexBuilder {
    pub fn allow_empty(mut self, value: Expression<bool>) -> Self {
        self.allow_empty = Some(value);
        self
    }
    pub fn label_id(mut self, value: Expression<String>) -> Self {
        self.label_id = Some(value);
        self
    }
    pub fn pattern(mut self, value: Expression<String>) -> Self {
        self.pattern = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn variable(mut self, value: String) -> Self {
        self.variable = Some(value);
        self
    }
    pub fn build(self) -> DivInputValidatorRegex {
        DivInputValidatorRegex {
            allow_empty: self.allow_empty.or_else(|| Some(DivInputValidatorRegex::default_allow_empty())),
            label_id: self.label_id.expect("missing required field 'label_id'"),
            pattern: self.pattern.expect("missing required field 'pattern'"),
            r#type: String::from("regex"),
            variable: self.variable.expect("missing required field 'variable'"),
        }
    }
}
