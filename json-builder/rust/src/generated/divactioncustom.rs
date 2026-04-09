// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionCustom {
    #[serde(default = "DivActionCustom::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionCustomBuilder {
    pub r#type: Option<String>,
}

impl DivActionCustom {
    pub fn builder() -> DivActionCustomBuilder {
        DivActionCustomBuilder::default()
    }
    pub fn default_type() -> String { "custom".to_string() }
}

impl DivActionCustomBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionCustom {
        DivActionCustom {
            r#type: String::from("custom"),
        }
    }
}
