// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivBlur {
    pub radius: Expression<i64>,
    #[serde(default = "DivBlur::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivBlurBuilder {
    pub radius: Option<Expression<i64>>,
    pub r#type: Option<String>,
}

impl DivBlur {
    pub fn builder() -> DivBlurBuilder {
        DivBlurBuilder::default()
    }
    pub fn default_type() -> String { "blur".to_string() }
}

impl DivBlurBuilder {
    pub fn radius(mut self, value: Expression<i64>) -> Self {
        self.radius = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivBlur {
        DivBlur {
            radius: self.radius.expect("missing required field 'radius'"),
            r#type: String::from("blur"),
        }
    }
}
