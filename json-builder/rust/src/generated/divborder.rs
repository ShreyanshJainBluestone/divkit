// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivBorder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corners_radius: Option<DivCornersRadius>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_shadow: Option<Expression<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<DivShadow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<DivStroke>,
}

#[derive(Debug, Clone, Default)]
pub struct DivBorderBuilder {
    pub corner_radius: Option<Expression<i64>>,
    pub corners_radius: Option<DivCornersRadius>,
    pub has_shadow: Option<Expression<bool>>,
    pub shadow: Option<DivShadow>,
    pub stroke: Option<DivStroke>,
}

impl DivBorder {
    pub fn builder() -> DivBorderBuilder {
        DivBorderBuilder::default()
    }
    pub fn default_has_shadow() -> Expression<bool> { Expression::value(true) }
}

impl DivBorderBuilder {
    pub fn corner_radius(mut self, value: Expression<i64>) -> Self {
        self.corner_radius = Some(value);
        self
    }
    pub fn corners_radius(mut self, value: DivCornersRadius) -> Self {
        self.corners_radius = Some(value);
        self
    }
    pub fn has_shadow(mut self, value: Expression<bool>) -> Self {
        self.has_shadow = Some(value);
        self
    }
    pub fn shadow(mut self, value: DivShadow) -> Self {
        self.shadow = Some(value);
        self
    }
    pub fn stroke(mut self, value: DivStroke) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn build(self) -> DivBorder {
        DivBorder {
            corner_radius: self.corner_radius,
            corners_radius: self.corners_radius,
            has_shadow: self.has_shadow.or_else(|| Some(DivBorder::default_has_shadow())),
            shadow: self.shadow,
            stroke: self.stroke,
        }
    }
}
