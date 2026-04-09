// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivCornersRadius {
    #[serde(rename = "bottom-left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_left: Option<Expression<i64>>,
    #[serde(rename = "bottom-right")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_right: Option<Expression<i64>>,
    #[serde(rename = "top-left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_left: Option<Expression<i64>>,
    #[serde(rename = "top-right")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_right: Option<Expression<i64>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivCornersRadiusBuilder {
    pub bottom_left: Option<Expression<i64>>,
    pub bottom_right: Option<Expression<i64>>,
    pub top_left: Option<Expression<i64>>,
    pub top_right: Option<Expression<i64>>,
}

impl DivCornersRadius {
    pub fn builder() -> DivCornersRadiusBuilder {
        DivCornersRadiusBuilder::default()
    }
}

impl DivCornersRadiusBuilder {
    pub fn bottom_left(mut self, value: Expression<i64>) -> Self {
        self.bottom_left = Some(value);
        self
    }
    pub fn bottom_right(mut self, value: Expression<i64>) -> Self {
        self.bottom_right = Some(value);
        self
    }
    pub fn top_left(mut self, value: Expression<i64>) -> Self {
        self.top_left = Some(value);
        self
    }
    pub fn top_right(mut self, value: Expression<i64>) -> Self {
        self.top_right = Some(value);
        self
    }
    pub fn build(self) -> DivCornersRadius {
        DivCornersRadius {
            bottom_left: self.bottom_left,
            bottom_right: self.bottom_right,
            top_left: self.top_left,
            top_right: self.top_right,
        }
    }
}
