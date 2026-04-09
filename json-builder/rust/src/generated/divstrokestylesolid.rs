// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivStrokeStyleSolid {
    #[serde(default = "DivStrokeStyleSolid::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivStrokeStyleSolidBuilder {
    pub r#type: Option<String>,
}

impl DivStrokeStyleSolid {
    pub fn builder() -> DivStrokeStyleSolidBuilder {
        DivStrokeStyleSolidBuilder::default()
    }
    pub fn default_type() -> String { "solid".to_string() }
}

impl DivStrokeStyleSolidBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivStrokeStyleSolid {
        DivStrokeStyleSolid {
            r#type: String::from("solid"),
        }
    }
}
