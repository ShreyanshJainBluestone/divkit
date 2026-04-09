// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFixedLengthInputMask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_visible: Option<Expression<bool>>,
    pub pattern: Expression<String>,
    pub pattern_elements: Vec<DivFixedLengthInputMaskPatternElement>,
    pub raw_text_variable: String,
    #[serde(default = "DivFixedLengthInputMask::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivFixedLengthInputMaskBuilder {
    pub always_visible: Option<Expression<bool>>,
    pub pattern: Option<Expression<String>>,
    pub pattern_elements: Option<Vec<DivFixedLengthInputMaskPatternElement>>,
    pub raw_text_variable: Option<String>,
    pub r#type: Option<String>,
}

impl DivFixedLengthInputMask {
    pub fn builder() -> DivFixedLengthInputMaskBuilder {
        DivFixedLengthInputMaskBuilder::default()
    }
    pub fn default_always_visible() -> Expression<bool> { Expression::value(true) }
    pub fn default_type() -> String { "fixed_length".to_string() }
}

impl DivFixedLengthInputMaskBuilder {
    pub fn always_visible(mut self, value: Expression<bool>) -> Self {
        self.always_visible = Some(value);
        self
    }
    pub fn pattern(mut self, value: Expression<String>) -> Self {
        self.pattern = Some(value);
        self
    }
    pub fn pattern_elements(mut self, value: Vec<DivFixedLengthInputMaskPatternElement>) -> Self {
        self.pattern_elements = Some(value);
        self
    }
    pub fn raw_text_variable(mut self, value: String) -> Self {
        self.raw_text_variable = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivFixedLengthInputMask {
        DivFixedLengthInputMask {
            always_visible: self.always_visible.or_else(|| Some(DivFixedLengthInputMask::default_always_visible())),
            pattern: self.pattern.expect("missing required field 'pattern'"),
            pattern_elements: self.pattern_elements.expect("missing required field 'pattern_elements'"),
            raw_text_variable: self.raw_text_variable.expect("missing required field 'raw_text_variable'"),
            r#type: String::from("fixed_length"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivFixedLengthInputMaskPatternElement {
    pub key: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Expression<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<Expression<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivFixedLengthInputMaskPatternElementBuilder {
    pub key: Option<Expression<String>>,
    pub placeholder: Option<Expression<String>>,
    pub regex: Option<Expression<String>>,
}

impl DivFixedLengthInputMaskPatternElement {
    pub fn builder() -> DivFixedLengthInputMaskPatternElementBuilder {
        DivFixedLengthInputMaskPatternElementBuilder::default()
    }
    pub fn default_placeholder() -> Expression<String> { Expression::value("_".to_string()) }
}

impl DivFixedLengthInputMaskPatternElementBuilder {
    pub fn key(mut self, value: Expression<String>) -> Self {
        self.key = Some(value);
        self
    }
    pub fn placeholder(mut self, value: Expression<String>) -> Self {
        self.placeholder = Some(value);
        self
    }
    pub fn regex(mut self, value: Expression<String>) -> Self {
        self.regex = Some(value);
        self
    }
    pub fn build(self) -> DivFixedLengthInputMaskPatternElement {
        DivFixedLengthInputMaskPatternElement {
            key: self.key.expect("missing required field 'key'"),
            placeholder: self.placeholder.or_else(|| Some(DivFixedLengthInputMaskPatternElement::default_placeholder())),
            regex: self.regex,
        }
    }
}
