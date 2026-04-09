// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivColorAnimator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Expression<DivAnimationDirection>>,
    pub duration: Expression<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_actions: Option<Vec<DivAction>>,
    pub end_value: Expression<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<DivCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delay: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<Expression<String>>,
    #[serde(default = "DivColorAnimator::default_type")]
    pub r#type: String,
    pub variable_name: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivColorAnimatorBuilder {
    pub cancel_actions: Option<Vec<DivAction>>,
    pub direction: Option<Expression<DivAnimationDirection>>,
    pub duration: Option<Expression<i64>>,
    pub end_actions: Option<Vec<DivAction>>,
    pub end_value: Option<Expression<String>>,
    pub id: Option<String>,
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    pub repeat_count: Option<DivCount>,
    pub start_delay: Option<Expression<i64>>,
    pub start_value: Option<Expression<String>>,
    pub r#type: Option<String>,
    pub variable_name: Option<String>,
}

impl DivColorAnimator {
    pub fn builder() -> DivColorAnimatorBuilder {
        DivColorAnimatorBuilder::default()
    }
    pub fn default_direction() -> Expression<DivAnimationDirection> { Expression::value(serde_json::from_str(r#""normal""#).unwrap()) }
    pub fn default_interpolator() -> Expression<DivAnimationInterpolator> { Expression::value(serde_json::from_str(r#""linear""#).unwrap()) }
    pub fn default_repeat_count() -> DivCount { serde_json::from_str(r#"{"type": "fixed", "value": 1}"#).unwrap() }
    pub fn default_start_delay() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_type() -> String { "color_animator".to_string() }
}

impl DivColorAnimatorBuilder {
    pub fn cancel_actions(mut self, value: Vec<DivAction>) -> Self {
        self.cancel_actions = Some(value);
        self
    }
    pub fn direction(mut self, value: Expression<DivAnimationDirection>) -> Self {
        self.direction = Some(value);
        self
    }
    pub fn duration(mut self, value: Expression<i64>) -> Self {
        self.duration = Some(value);
        self
    }
    pub fn end_actions(mut self, value: Vec<DivAction>) -> Self {
        self.end_actions = Some(value);
        self
    }
    pub fn end_value(mut self, value: Expression<String>) -> Self {
        self.end_value = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn interpolator(mut self, value: Expression<DivAnimationInterpolator>) -> Self {
        self.interpolator = Some(value);
        self
    }
    pub fn repeat_count(mut self, value: DivCount) -> Self {
        self.repeat_count = Some(value);
        self
    }
    pub fn start_delay(mut self, value: Expression<i64>) -> Self {
        self.start_delay = Some(value);
        self
    }
    pub fn start_value(mut self, value: Expression<String>) -> Self {
        self.start_value = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn variable_name(mut self, value: String) -> Self {
        self.variable_name = Some(value);
        self
    }
    pub fn build(self) -> DivColorAnimator {
        DivColorAnimator {
            cancel_actions: self.cancel_actions,
            direction: self.direction.or_else(|| Some(DivColorAnimator::default_direction())),
            duration: self.duration.expect("missing required field 'duration'"),
            end_actions: self.end_actions,
            end_value: self.end_value.expect("missing required field 'end_value'"),
            id: self.id.expect("missing required field 'id'"),
            interpolator: self.interpolator.or_else(|| Some(DivColorAnimator::default_interpolator())),
            repeat_count: self.repeat_count.or_else(|| Some(DivColorAnimator::default_repeat_count())),
            start_delay: self.start_delay.or_else(|| Some(DivColorAnimator::default_start_delay())),
            start_value: self.start_value,
            r#type: String::from("color_animator"),
            variable_name: self.variable_name.expect("missing required field 'variable_name'"),
        }
    }
}
