// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPageContentSize {
    #[serde(default = "DivPageContentSize::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivPageContentSizeBuilder {
    pub r#type: Option<String>,
}

impl DivPageContentSize {
    pub fn builder() -> DivPageContentSizeBuilder {
        DivPageContentSizeBuilder::default()
    }
    pub fn default_type() -> String { "wrap_content".to_string() }
}

impl DivPageContentSizeBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivPageContentSize {
        DivPageContentSize {
            r#type: String::from("wrap_content"),
        }
    }
}
