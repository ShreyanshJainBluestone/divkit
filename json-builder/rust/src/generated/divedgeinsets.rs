// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivEdgeInsets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Expression<DivSizeUnit>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivEdgeInsetsBuilder {
    pub bottom: Option<Expression<i64>>,
    pub end: Option<Expression<i64>>,
    pub left: Option<Expression<i64>>,
    pub right: Option<Expression<i64>>,
    pub start: Option<Expression<i64>>,
    pub top: Option<Expression<i64>>,
    pub unit: Option<Expression<DivSizeUnit>>,
}

impl DivEdgeInsets {
    pub fn builder() -> DivEdgeInsetsBuilder {
        DivEdgeInsetsBuilder::default()
    }
    pub fn default_bottom() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_left() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_right() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_top() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_unit() -> Expression<DivSizeUnit> { Expression::value(serde_json::from_str(r#""dp""#).unwrap()) }
}

impl DivEdgeInsetsBuilder {
    pub fn bottom(mut self, value: Expression<i64>) -> Self {
        self.bottom = Some(value);
        self
    }
    pub fn end(mut self, value: Expression<i64>) -> Self {
        self.end = Some(value);
        self
    }
    pub fn left(mut self, value: Expression<i64>) -> Self {
        self.left = Some(value);
        self
    }
    pub fn right(mut self, value: Expression<i64>) -> Self {
        self.right = Some(value);
        self
    }
    pub fn start(mut self, value: Expression<i64>) -> Self {
        self.start = Some(value);
        self
    }
    pub fn top(mut self, value: Expression<i64>) -> Self {
        self.top = Some(value);
        self
    }
    pub fn unit(mut self, value: Expression<DivSizeUnit>) -> Self {
        self.unit = Some(value);
        self
    }
    pub fn build(self) -> DivEdgeInsets {
        DivEdgeInsets {
            bottom: self.bottom.or_else(|| Some(DivEdgeInsets::default_bottom())),
            end: self.end,
            left: self.left.or_else(|| Some(DivEdgeInsets::default_left())),
            right: self.right.or_else(|| Some(DivEdgeInsets::default_right())),
            start: self.start,
            top: self.top.or_else(|| Some(DivEdgeInsets::default_top())),
            unit: self.unit.or_else(|| Some(DivEdgeInsets::default_unit())),
        }
    }
}
