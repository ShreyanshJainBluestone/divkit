// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivLinearGradient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_map: Option<Vec<DivLinearGradientColorPoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<String>>,
    #[serde(default = "DivLinearGradient::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivLinearGradientBuilder {
    pub angle: Option<Expression<i64>>,
    pub color_map: Option<Vec<DivLinearGradientColorPoint>>,
    pub colors: Option<Vec<String>>,
    pub r#type: Option<String>,
}

impl DivLinearGradient {
    pub fn builder() -> DivLinearGradientBuilder {
        DivLinearGradientBuilder::default()
    }
    pub fn default_angle() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_type() -> String { "gradient".to_string() }
}

impl DivLinearGradientBuilder {
    pub fn angle(mut self, value: Expression<i64>) -> Self {
        self.angle = Some(value);
        self
    }
    pub fn color_map(mut self, value: Vec<DivLinearGradientColorPoint>) -> Self {
        self.color_map = Some(value);
        self
    }
    pub fn colors(mut self, value: Vec<String>) -> Self {
        self.colors = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivLinearGradient {
        DivLinearGradient {
            angle: self.angle.or_else(|| Some(DivLinearGradient::default_angle())),
            color_map: self.color_map,
            colors: self.colors,
            r#type: String::from("gradient"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivLinearGradientColorPoint {
    pub color: Expression<String>,
    pub position: Expression<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivLinearGradientColorPointBuilder {
    pub color: Option<Expression<String>>,
    pub position: Option<Expression<f64>>,
}

impl DivLinearGradientColorPoint {
    pub fn builder() -> DivLinearGradientColorPointBuilder {
        DivLinearGradientColorPointBuilder::default()
    }
}

impl DivLinearGradientColorPointBuilder {
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn position(mut self, value: Expression<f64>) -> Self {
        self.position = Some(value);
        self
    }
    pub fn build(self) -> DivLinearGradientColorPoint {
        DivLinearGradientColorPoint {
            color: self.color.expect("missing required field 'color'"),
            position: self.position.expect("missing required field 'position'"),
        }
    }
}
