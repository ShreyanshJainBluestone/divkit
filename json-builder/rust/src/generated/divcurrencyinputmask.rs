// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivCurrencyInputMask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<Expression<String>>,
    pub raw_text_variable: String,
    #[serde(default = "DivCurrencyInputMask::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivCurrencyInputMaskBuilder {
    pub locale: Option<Expression<String>>,
    pub raw_text_variable: Option<String>,
    pub r#type: Option<String>,
}

impl DivCurrencyInputMask {
    pub fn builder() -> DivCurrencyInputMaskBuilder {
        DivCurrencyInputMaskBuilder::default()
    }
    pub fn default_type() -> String { "currency".to_string() }
}

impl DivCurrencyInputMaskBuilder {
    pub fn locale(mut self, value: Expression<String>) -> Self {
        self.locale = Some(value);
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
    pub fn build(self) -> DivCurrencyInputMask {
        DivCurrencyInputMask {
            locale: self.locale,
            raw_text_variable: self.raw_text_variable.expect("missing required field 'raw_text_variable'"),
            r#type: String::from("currency"),
        }
    }
}
