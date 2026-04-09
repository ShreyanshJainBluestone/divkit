// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPageTransformationOverlap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_alpha: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_scale: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_alpha: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_scale: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversed_stacking_order: Option<Expression<bool>>,
    #[serde(default = "DivPageTransformationOverlap::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivPageTransformationOverlapBuilder {
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    pub next_page_alpha: Option<Expression<f64>>,
    pub next_page_scale: Option<Expression<f64>>,
    pub previous_page_alpha: Option<Expression<f64>>,
    pub previous_page_scale: Option<Expression<f64>>,
    pub reversed_stacking_order: Option<Expression<bool>>,
    pub r#type: Option<String>,
}

impl DivPageTransformationOverlap {
    pub fn builder() -> DivPageTransformationOverlapBuilder {
        DivPageTransformationOverlapBuilder::default()
    }
    pub fn default_interpolator() -> Expression<DivAnimationInterpolator> { Expression::value(serde_json::from_str(r#""ease_in_out""#).unwrap()) }
    pub fn default_next_page_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_next_page_scale() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_previous_page_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_previous_page_scale() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_reversed_stacking_order() -> Expression<bool> { Expression::value(true) }
    pub fn default_type() -> String { "overlap".to_string() }
}

impl DivPageTransformationOverlapBuilder {
    pub fn interpolator(mut self, value: Expression<DivAnimationInterpolator>) -> Self {
        self.interpolator = Some(value);
        self
    }
    pub fn next_page_alpha(mut self, value: Expression<f64>) -> Self {
        self.next_page_alpha = Some(value);
        self
    }
    pub fn next_page_scale(mut self, value: Expression<f64>) -> Self {
        self.next_page_scale = Some(value);
        self
    }
    pub fn previous_page_alpha(mut self, value: Expression<f64>) -> Self {
        self.previous_page_alpha = Some(value);
        self
    }
    pub fn previous_page_scale(mut self, value: Expression<f64>) -> Self {
        self.previous_page_scale = Some(value);
        self
    }
    pub fn reversed_stacking_order(mut self, value: Expression<bool>) -> Self {
        self.reversed_stacking_order = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivPageTransformationOverlap {
        DivPageTransformationOverlap {
            interpolator: self.interpolator.or_else(|| Some(DivPageTransformationOverlap::default_interpolator())),
            next_page_alpha: self.next_page_alpha.or_else(|| Some(DivPageTransformationOverlap::default_next_page_alpha())),
            next_page_scale: self.next_page_scale.or_else(|| Some(DivPageTransformationOverlap::default_next_page_scale())),
            previous_page_alpha: self.previous_page_alpha.or_else(|| Some(DivPageTransformationOverlap::default_previous_page_alpha())),
            previous_page_scale: self.previous_page_scale.or_else(|| Some(DivPageTransformationOverlap::default_previous_page_scale())),
            reversed_stacking_order: self.reversed_stacking_order.or_else(|| Some(DivPageTransformationOverlap::default_reversed_stacking_order())),
            r#type: String::from("overlap"),
        }
    }
}
