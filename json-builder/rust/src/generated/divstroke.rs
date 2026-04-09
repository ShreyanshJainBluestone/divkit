// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivStroke {
    pub color: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<DivStrokeStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Expression<DivSizeUnit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Expression<f64>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivStrokeBuilder {
    pub color: Option<Expression<String>>,
    pub style: Option<DivStrokeStyle>,
    pub unit: Option<Expression<DivSizeUnit>>,
    pub width: Option<Expression<f64>>,
}

impl DivStroke {
    pub fn builder() -> DivStrokeBuilder {
        DivStrokeBuilder::default()
    }
    pub fn default_style() -> DivStrokeStyle { serde_json::from_str(r#"{"type": "solid"}"#).unwrap() }
    pub fn default_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""dp""#).unwrap()) }
    pub fn default_width() -> Expression<f64> { Expression::value(1_f64) }
}

impl DivStrokeBuilder {
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn style(mut self, value: DivStrokeStyle) -> Self {
        self.style = Some(value);
        self
    }
    pub fn unit(mut self, value: Expression<DivSizeUnit>) -> Self {
        self.unit = Some(value);
        self
    }
    pub fn width(mut self, value: Expression<f64>) -> Self {
        self.width = Some(value);
        self
    }
    pub fn build(self) -> DivStroke {
        DivStroke {
            color: self.color.expect("missing required field 'color'"),
            style: self.style.or_else(|| Some(DivStroke::default_style())),
            unit: self.unit.or_else(|| Some(DivStroke::default_unit())),
            width: self.width.or_else(|| Some(DivStroke::default_width())),
        }
    }
}
