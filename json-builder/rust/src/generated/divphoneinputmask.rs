// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPhoneInputMask {
    pub raw_text_variable: String,
    #[serde(default = "DivPhoneInputMask::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivPhoneInputMaskBuilder {
    pub raw_text_variable: Option<String>,
    pub r#type: Option<String>,
}

impl DivPhoneInputMask {
    pub fn builder() -> DivPhoneInputMaskBuilder {
        DivPhoneInputMaskBuilder::default()
    }
    pub fn default_type() -> String { "phone".to_string() }
}

impl DivPhoneInputMaskBuilder {
    pub fn raw_text_variable(mut self, value: String) -> Self {
        self.raw_text_variable = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivPhoneInputMask {
        DivPhoneInputMask {
            raw_text_variable: self.raw_text_variable.expect("missing required field 'raw_text_variable'"),
            r#type: String::from("phone"),
        }
    }
}
