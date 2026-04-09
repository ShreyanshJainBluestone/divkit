// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivAbsoluteEdgeInsets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<Expression<i64>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivAbsoluteEdgeInsetsBuilder {
    pub bottom: Option<Expression<i64>>,
    pub left: Option<Expression<i64>>,
    pub right: Option<Expression<i64>>,
    pub top: Option<Expression<i64>>,
}

impl DivAbsoluteEdgeInsets {
    pub fn builder() -> DivAbsoluteEdgeInsetsBuilder {
        DivAbsoluteEdgeInsetsBuilder::default()
    }
    pub fn default_bottom() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_left() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_right() -> Expression<i64> { Expression::value(0i64) }
    pub fn default_top() -> Expression<i64> { Expression::value(0i64) }
}

impl DivAbsoluteEdgeInsetsBuilder {
    pub fn bottom(mut self, value: Expression<i64>) -> Self {
        self.bottom = Some(value);
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
    pub fn top(mut self, value: Expression<i64>) -> Self {
        self.top = Some(value);
        self
    }
    pub fn build(self) -> DivAbsoluteEdgeInsets {
        DivAbsoluteEdgeInsets {
            bottom: self.bottom.or_else(|| Some(DivAbsoluteEdgeInsets::default_bottom())),
            left: self.left.or_else(|| Some(DivAbsoluteEdgeInsets::default_left())),
            right: self.right.or_else(|| Some(DivAbsoluteEdgeInsets::default_right())),
            top: self.top.or_else(|| Some(DivAbsoluteEdgeInsets::default_top())),
        }
    }
}
