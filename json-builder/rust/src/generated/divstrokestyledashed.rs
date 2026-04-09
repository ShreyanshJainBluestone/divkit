// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivStrokeStyleDashed {
    #[serde(default = "DivStrokeStyleDashed::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivStrokeStyleDashedBuilder {
    pub r#type: Option<String>,
}

impl DivStrokeStyleDashed {
    pub fn builder() -> DivStrokeStyleDashedBuilder {
        DivStrokeStyleDashedBuilder::default()
    }
    pub fn default_type() -> String { "dashed".to_string() }
}

impl DivStrokeStyleDashedBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivStrokeStyleDashed {
        DivStrokeStyleDashed {
            r#type: String::from("dashed"),
        }
    }
}
