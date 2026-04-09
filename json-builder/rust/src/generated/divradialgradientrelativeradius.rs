// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivRadialGradientRelativeRadius {
    #[serde(default = "DivRadialGradientRelativeRadius::default_type")]
    pub r#type: String,
    pub value: Expression<DivRadialGradientRelativeRadiusValue>,
}

#[derive(Debug, Clone, Default)]
pub struct DivRadialGradientRelativeRadiusBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<DivRadialGradientRelativeRadiusValue>>,
}

impl DivRadialGradientRelativeRadius {
    pub fn builder() -> DivRadialGradientRelativeRadiusBuilder {
        DivRadialGradientRelativeRadiusBuilder::default()
    }
    pub fn default_type() -> String { "relative".to_string() }
}

impl DivRadialGradientRelativeRadiusBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<DivRadialGradientRelativeRadiusValue>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DivRadialGradientRelativeRadius {
        DivRadialGradientRelativeRadius {
            r#type: String::from("relative"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivRadialGradientRelativeRadiusValue {
    #[serde(rename = "nearest_corner")]
    NearestCorner,
    #[serde(rename = "farthest_corner")]
    FarthestCorner,
    #[serde(rename = "nearest_side")]
    NearestSide,
    #[serde(rename = "farthest_side")]
    FarthestSide,
}
