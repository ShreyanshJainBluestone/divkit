// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivSlideTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<DivDimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge: Option<Expression<DivSlideTransitionEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_delay: Option<Expression<i64>>,
    #[serde(default = "DivSlideTransition::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivSlideTransitionBuilder {
    pub distance: Option<DivDimension>,
    pub duration: Option<Expression<i64>>,
    pub edge: Option<Expression<DivSlideTransitionEdge>>,
    pub interpolator: Option<Expression<DivAnimationInterpolator>>,
    pub start_delay: Option<Expression<i64>>,
    pub r#type: Option<String>,
}

impl DivSlideTransition {
    pub fn builder() -> DivSlideTransitionBuilder {
        DivSlideTransitionBuilder::default()
    }
    pub fn default_duration() -> Expression<i64> { Expression::value(200i64) }
    pub fn default_edge() -> Expression<DivSlideTransitionEdge> { Expression::value(serde_json::from_str(r#""bottom""#).unwrap()) }
    pub fn default_interpolator() -> Expression<DivAnimationInterpolator> { Expression::value(serde_json::from_str(r#""ease_in_out""#).unwrap()) }
    pub fn default_start_delay() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_type() -> String { "slide".to_string() }
}

impl DivSlideTransitionBuilder {
    pub fn distance(mut self, value: DivDimension) -> Self {
        self.distance = Some(value);
        self
    }
    pub fn duration(mut self, value: Expression<i64>) -> Self {
        self.duration = Some(value);
        self
    }
    pub fn edge(mut self, value: Expression<DivSlideTransitionEdge>) -> Self {
        self.edge = Some(value);
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
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivSlideTransition {
        DivSlideTransition {
            distance: self.distance,
            duration: self.duration.or_else(|| Some(DivSlideTransition::default_duration())),
            edge: self.edge.or_else(|| Some(DivSlideTransition::default_edge())),
            interpolator: self.interpolator.or_else(|| Some(DivSlideTransition::default_interpolator())),
            start_delay: self.start_delay.or_else(|| Some(DivSlideTransition::default_start_delay())),
            r#type: String::from("slide"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivSlideTransitionEdge {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "bottom")]
    Bottom,
}
