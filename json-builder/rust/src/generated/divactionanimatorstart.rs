// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionAnimatorStart {
    pub animator_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Expression<DivAnimationDirection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value: Option<DivTypedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<DivCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delay: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<DivTypedValue>,
    #[serde(default = "DivActionAnimatorStart::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionAnimatorStartBuilder {
    pub animator_id: Option<String>,
    pub direction: Option<Expression<DivAnimationDirection>>,
    pub duration: Option<Expression<i64>>,
    pub end_value: Option<DivTypedValue>,
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    pub repeat_count: Option<DivCount>,
    pub start_delay: Option<Expression<i64>>,
    pub start_value: Option<DivTypedValue>,
    pub r#type: Option<String>,
}

impl DivActionAnimatorStart {
    pub fn builder() -> DivActionAnimatorStartBuilder {
        DivActionAnimatorStartBuilder::default()
    }
    pub fn default_type() -> String { "animator_start".to_string() }
}

impl DivActionAnimatorStartBuilder {
    pub fn animator_id(mut self, value: String) -> Self {
        self.animator_id = Some(value);
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
    pub fn end_value(mut self, value: DivTypedValue) -> Self {
        self.end_value = Some(value);
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
    pub fn start_value(mut self, value: DivTypedValue) -> Self {
        self.start_value = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionAnimatorStart {
        DivActionAnimatorStart {
            animator_id: self.animator_id.expect("missing required field 'animator_id'"),
            direction: self.direction,
            duration: self.duration,
            end_value: self.end_value,
            interpolator: self.interpolator,
            repeat_count: self.repeat_count,
            start_delay: self.start_delay,
            start_value: self.start_value,
            r#type: String::from("animator_start"),
        }
    }
}
