// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTransitionBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delay: Option<Expression<i64>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTransitionBaseBuilder {
    pub duration: Option<Expression<i64>>,
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    pub start_delay: Option<Expression<i64>>,
}

impl DivTransitionBase {
    pub fn builder() -> DivTransitionBaseBuilder {
        DivTransitionBaseBuilder::default()
    }
    pub fn default_duration() -> Expression<i64> { Expression::value(200i64) }
    pub fn default_interpolator() -> Expression<DivAnimationInterpolator> { Expression::value(serde_json::from_str(r#""ease_in_out""#).unwrap()) }
    pub fn default_start_delay() -> Expression<i64> { Expression::value(0i64) }
}

impl DivTransitionBaseBuilder {
    pub fn duration(mut self, value: Expression<i64>) -> Self {
        self.duration = Some(value);
        self
    }
    pub fn interpolator(mut self, value: Expression<DivAnimationInterpolator>) -> Self {
        self.interpolator = Some(value);
        self
    }
    pub fn start_delay(mut self, value: Expression<i64>) -> Self {
        self.start_delay = Some(value);
        self
    }
    pub fn build(self) -> DivTransitionBase {
        DivTransitionBase {
            duration: self.duration.or_else(|| Some(DivTransitionBase::default_duration())),
            interpolator: self.interpolator.or_else(|| Some(DivTransitionBase::default_interpolator())),
            start_delay: self.start_delay.or_else(|| Some(DivTransitionBase::default_start_delay())),
        }
    }
}
