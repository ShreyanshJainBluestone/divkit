// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivInputMaskBase {
    pub raw_text_variable: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivInputMaskBaseBuilder {
    pub raw_text_variable: Option<String>,
}

impl DivInputMaskBase {
    pub fn builder() -> DivInputMaskBaseBuilder {
        DivInputMaskBaseBuilder::default()
    }
}

impl DivInputMaskBaseBuilder {
    pub fn raw_text_variable(mut self, value: String) -> Self {
        self.raw_text_variable = Some(value);
        self
    }
    pub fn build(self) -> DivInputMaskBase {
        DivInputMaskBase {
            raw_text_variable: self.raw_text_variable.expect("missing required field 'raw_text_variable'"),
        }
    }
}
