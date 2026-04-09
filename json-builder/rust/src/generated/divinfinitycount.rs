// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivInfinityCount {
    #[serde(default = "DivInfinityCount::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivInfinityCountBuilder {
    pub r#type: Option<String>,
}

impl DivInfinityCount {
    pub fn builder() -> DivInfinityCountBuilder {
        DivInfinityCountBuilder::default()
    }
    pub fn default_type() -> String { "infinity".to_string() }
}

impl DivInfinityCountBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivInfinityCount {
        DivInfinityCount {
            r#type: String::from("infinity"),
        }
    }
}
