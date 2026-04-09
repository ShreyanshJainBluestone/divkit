// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPageTransformationSlide {
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
    #[serde(default = "DivPageTransformationSlide::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivPageTransformationSlideBuilder {
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    pub next_page_alpha: Option<Expression<f64>>,
    pub next_page_scale: Option<Expression<f64>>,
    pub previous_page_alpha: Option<Expression<f64>>,
    pub previous_page_scale: Option<Expression<f64>>,
    pub r#type: Option<String>,
}

impl DivPageTransformationSlide {
    pub fn builder() -> DivPageTransformationSlideBuilder {
        DivPageTransformationSlideBuilder::default()
    }
    pub fn default_interpolator() -> Expression<DivAnimationInterpolator> { Expression::value(serde_json::from_str(r#""ease_in_out""#).unwrap()) }
    pub fn default_next_page_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_next_page_scale() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_previous_page_alpha() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_previous_page_scale() -> Expression<f64> { Expression::value(1.0_f64) }
    pub fn default_type() -> String { "slide".to_string() }
}

impl DivPageTransformationSlideBuilder {
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
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivPageTransformationSlide {
        DivPageTransformationSlide {
            interpolator: self.interpolator.or_else(|| Some(DivPageTransformationSlide::default_interpolator())),
            next_page_alpha: self.next_page_alpha.or_else(|| Some(DivPageTransformationSlide::default_next_page_alpha())),
            next_page_scale: self.next_page_scale.or_else(|| Some(DivPageTransformationSlide::default_next_page_scale())),
            previous_page_alpha: self.previous_page_alpha.or_else(|| Some(DivPageTransformationSlide::default_previous_page_alpha())),
            previous_page_scale: self.previous_page_scale.or_else(|| Some(DivPageTransformationSlide::default_previous_page_scale())),
            r#type: String::from("slide"),
        }
    }
}
