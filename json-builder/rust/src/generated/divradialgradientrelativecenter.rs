// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivRadialGradientRelativeCenter {
    #[serde(default = "DivRadialGradientRelativeCenter::default_type")]
    pub r#type: String,
    pub value: Expression<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivRadialGradientRelativeCenterBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<f64>>,
}

impl DivRadialGradientRelativeCenter {
    pub fn builder() -> DivRadialGradientRelativeCenterBuilder {
        DivRadialGradientRelativeCenterBuilder::default()
    }
    pub fn default_type() -> String { "relative".to_string() }
}

impl DivRadialGradientRelativeCenterBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<f64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DivRadialGradientRelativeCenter {
        DivRadialGradientRelativeCenter {
            r#type: String::from("relative"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
