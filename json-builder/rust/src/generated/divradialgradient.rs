// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivRadialGradient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_x: Option<DivRadialGradientCenter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_y: Option<DivRadialGradientCenter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_map: Option<Vec<DivRadialGradientColorPoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<DivRadialGradientRadius>,
    #[serde(default = "DivRadialGradient::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivRadialGradientBuilder {
    pub center_x: Option<DivRadialGradientCenter>,
    pub center_y: Option<DivRadialGradientCenter>,
    pub color_map: Option<Vec<DivRadialGradientColorPoint>>,
    pub colors: Option<Vec<String>>,
    pub radius: Option<DivRadialGradientRadius>,
    pub r#type: Option<String>,
}

impl DivRadialGradient {
    pub fn builder() -> DivRadialGradientBuilder {
        DivRadialGradientBuilder::default()
    }
    pub fn default_center_x() -> DivRadialGradientCenter { serde_json::from_str(r#"{"type": "relative", "value": 0.5 }"#).unwrap() }
    pub fn default_center_y() -> DivRadialGradientCenter { serde_json::from_str(r#"{"type": "relative", "value": 0.5 }"#).unwrap() }
    pub fn default_radius() -> DivRadialGradientRadius { serde_json::from_str(r#"{"type": "relative", "value": "farthest_corner" }"#).unwrap() }
    pub fn default_type() -> String { "radial_gradient".to_string() }
}

impl DivRadialGradientBuilder {
    pub fn center_x(mut self, value: DivRadialGradientCenter) -> Self {
        self.center_x = Some(value);
        self
    }
    pub fn center_y(mut self, value: DivRadialGradientCenter) -> Self {
        self.center_y = Some(value);
        self
    }
    pub fn color_map(mut self, value: Vec<DivRadialGradientColorPoint>) -> Self {
        self.color_map = Some(value);
        self
    }
    pub fn colors(mut self, value: Vec<String>) -> Self {
        self.colors = Some(value);
        self
    }
    pub fn radius(mut self, value: DivRadialGradientRadius) -> Self {
        self.radius = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivRadialGradient {
        DivRadialGradient {
            center_x: self.center_x.or_else(|| Some(DivRadialGradient::default_center_x())),
            center_y: self.center_y.or_else(|| Some(DivRadialGradient::default_center_y())),
            color_map: self.color_map,
            colors: self.colors,
            radius: self.radius.or_else(|| Some(DivRadialGradient::default_radius())),
            r#type: String::from("radial_gradient"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivRadialGradientColorPoint {
    pub color: Expression<String>,
    pub position: Expression<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivRadialGradientColorPointBuilder {
    pub color: Option<Expression<String>>,
    pub position: Option<Expression<f64>>,
}

impl DivRadialGradientColorPoint {
    pub fn builder() -> DivRadialGradientColorPointBuilder {
        DivRadialGradientColorPointBuilder::default()
    }
}

impl DivRadialGradientColorPointBuilder {
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn position(mut self, value: Expression<f64>) -> Self {
        self.position = Some(value);
        self
    }
    pub fn build(self) -> DivRadialGradientColorPoint {
        DivRadialGradientColorPoint {
            color: self.color.expect("missing required field 'color'"),
            position: self.position.expect("missing required field 'position'"),
        }
    }
}
