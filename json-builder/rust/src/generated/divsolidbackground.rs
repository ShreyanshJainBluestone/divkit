// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivSolidBackground {
    pub color: Expression<String>,
    #[serde(default = "DivSolidBackground::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivSolidBackgroundBuilder {
    pub color: Option<Expression<String>>,
    pub r#type: Option<String>,
}

impl DivSolidBackground {
    pub fn builder() -> DivSolidBackgroundBuilder {
        DivSolidBackgroundBuilder::default()
    }
    pub fn default_type() -> String { "solid".to_string() }
}

impl DivSolidBackgroundBuilder {
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivSolidBackground {
        DivSolidBackground {
            color: self.color.expect("missing required field 'color'"),
            r#type: String::from("solid"),
        }
    }
}
