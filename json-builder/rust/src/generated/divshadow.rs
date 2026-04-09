// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivShadow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Expression<String>>,
    pub offset: DivPoint,
}

#[derive(Debug, Clone, Default)]
pub struct DivShadowBuilder {
    pub alpha: Option<Expression<f64>>,
    pub blur: Option<Expression<i64>>,
    pub color: Option<Expression<String>>,
    pub offset: Option<DivPoint>,
}

impl DivShadow {
    pub fn builder() -> DivShadowBuilder {
        DivShadowBuilder::default()
    }
    pub fn default_alpha() -> Expression<f64> { Expression::value(0.19_f64) }
    pub fn default_blur() -> Expression<i64> { Expression::value(2i64) }
    pub fn default_color() -> Expression<String> { Expression::value("#000000".to_string()) }
}

impl DivShadowBuilder {
    pub fn alpha(mut self, value: Expression<f64>) -> Self {
        self.alpha = Some(value);
        self
    }
    pub fn blur(mut self, value: Expression<i64>) -> Self {
        self.blur = Some(value);
        self
    }
    pub fn color(mut self, value: Expression<String>) -> Self {
        self.color = Some(value);
        self
    }
    pub fn offset(mut self, value: DivPoint) -> Self {
        self.offset = Some(value);
        self
    }
    pub fn build(self) -> DivShadow {
        DivShadow {
            alpha: self.alpha.or_else(|| Some(DivShadow::default_alpha())),
            blur: self.blur.or_else(|| Some(DivShadow::default_blur())),
            color: self.color.or_else(|| Some(DivShadow::default_color())),
            offset: self.offset.expect("missing required field 'offset'"),
        }
    }
}
