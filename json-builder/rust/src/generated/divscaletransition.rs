// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivScaleTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_x: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_y: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delay: Option<Expression<i64>>,
    #[serde(default = "DivScaleTransition::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivScaleTransitionBuilder {
    pub duration: Option<Expression<i64>>,
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    pub pivot_x: Option<Expression<f64>>,
    pub pivot_y: Option<Expression<f64>>,
    pub scale: Option<Expression<f64>>,
    pub start_delay: Option<Expression<i64>>,
    pub r#type: Option<String>,
}

impl DivScaleTransition {
    pub fn builder() -> DivScaleTransitionBuilder {
        DivScaleTransitionBuilder::default()
    }
    pub fn default_duration() -> Expression<i64> { Expression::value(200i64) }
    pub fn default_interpolator() -> Expression<DivAnimationInterpolator> { Expression::value(serde_json::from_str(r#""ease_in_out""#).unwrap()) }
    pub fn default_pivot_x() -> Expression<f64> { Expression::value(0.5_f64) }
    pub fn default_pivot_y() -> Expression<f64> { Expression::value(0.5_f64) }
    pub fn default_scale() -> Expression<f64> { Expression::value(0.0_f64) }
    pub fn default_start_delay() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_type() -> String { "scale".to_string() }
}

impl DivScaleTransitionBuilder {
    pub fn duration(mut self, value: Expression<i64>) -> Self {
        self.duration = Some(value);
        self
    }
    pub fn interpolator(mut self, value: Expression<DivAnimationInterpolator>) -> Self {
        self.interpolator = Some(value);
        self
    }
    pub fn pivot_x(mut self, value: Expression<f64>) -> Self {
        self.pivot_x = Some(value);
        self
    }
    pub fn pivot_y(mut self, value: Expression<f64>) -> Self {
        self.pivot_y = Some(value);
        self
    }
    pub fn scale(mut self, value: Expression<f64>) -> Self {
        self.scale = Some(value);
        self
    }
    pub fn start_delay(mut self, value: Expression<i64>) -> Self {
        self.start_delay = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivScaleTransition {
        DivScaleTransition {
            duration: self.duration.or_else(|| Some(DivScaleTransition::default_duration())),
            interpolator: self.interpolator.or_else(|| Some(DivScaleTransition::default_interpolator())),
            pivot_x: self.pivot_x.or_else(|| Some(DivScaleTransition::default_pivot_x())),
            pivot_y: self.pivot_y.or_else(|| Some(DivScaleTransition::default_pivot_y())),
            scale: self.scale.or_else(|| Some(DivScaleTransition::default_scale())),
            start_delay: self.start_delay.or_else(|| Some(DivScaleTransition::default_start_delay())),
            r#type: String::from("scale"),
        }
    }
}
