// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivAnimation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value: Option<Expression<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DivAnimation>>,
    pub name: Expression<DivAnimationName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<DivCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delay: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<Expression<f64>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivAnimationBuilder {
    pub duration: Option<Expression<i64>>,
    pub end_value: Option<Expression<f64>>,
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    pub items: Option<Vec<DivAnimation>>,
    pub name: Option<Expression<DivAnimationName>>,
    pub repeat: Option<DivCount>,
    pub start_delay: Option<Expression<i64>>,
    pub start_value: Option<Expression<f64>>,
}

impl DivAnimation {
    pub fn builder() -> DivAnimationBuilder {
        DivAnimationBuilder::default()
    }
    pub fn default_duration() -> Expression<i64> { Expression::value(300i64) }
    pub fn default_interpolator() -> Expression<DivAnimationInterpolator> { Expression::value(serde_json::from_str(r#""spring""#).unwrap()) }
    pub fn default_repeat() -> DivCount { serde_json::from_str(r#"{ "type": "infinity" }"#).unwrap() }
    pub fn default_start_delay() -> Expression<i64> { Expression::value(0i64) }
}

impl DivAnimationBuilder {
    pub fn duration(mut self, value: Expression<i64>) -> Self {
        self.duration = Some(value);
        self
    }
    pub fn end_value(mut self, value: Expression<f64>) -> Self {
        self.end_value = Some(value);
        self
    }
    pub fn interpolator(mut self, value: Expression<DivAnimationInterpolator>) -> Self {
        self.interpolator = Some(value);
        self
    }
    pub fn items(mut self, value: Vec<DivAnimation>) -> Self {
        self.items = Some(value);
        self
    }
    pub fn name(mut self, value: Expression<DivAnimationName>) -> Self {
        self.name = Some(value);
        self
    }
    pub fn repeat(mut self, value: DivCount) -> Self {
        self.repeat = Some(value);
        self
    }
    pub fn start_delay(mut self, value: Expression<i64>) -> Self {
        self.start_delay = Some(value);
        self
    }
    pub fn start_value(mut self, value: Expression<f64>) -> Self {
        self.start_value = Some(value);
        self
    }
    pub fn build(self) -> DivAnimation {
        DivAnimation {
            duration: self.duration.or_else(|| Some(DivAnimation::default_duration())),
            end_value: self.end_value,
            interpolator: self.interpolator.or_else(|| Some(DivAnimation::default_interpolator())),
            items: self.items,
            name: self.name.expect("missing required field 'name'"),
            repeat: self.repeat.or_else(|| Some(DivAnimation::default_repeat())),
            start_delay: self.start_delay.or_else(|| Some(DivAnimation::default_start_delay())),
            start_value: self.start_value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivAnimationName {
    #[serde(rename = "fade")]
    Fade,
    #[serde(rename = "translate")]
    Translate,
    #[serde(rename = "scale")]
    Scale,
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "no_animation")]
    NoAnimation,
}
