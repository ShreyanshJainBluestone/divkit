// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTextRangeBorder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<DivStroke>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTextRangeBorderBuilder {
    pub corner_radius: Option<Expression<i64>>,
    pub stroke: Option<DivStroke>,
}

impl DivTextRangeBorder {
    pub fn builder() -> DivTextRangeBorderBuilder {
        DivTextRangeBorderBuilder::default()
    }
}

impl DivTextRangeBorderBuilder {
    pub fn corner_radius(mut self, value: Expression<i64>) -> Self {
        self.corner_radius = Some(value);
        self
    }
    pub fn stroke(mut self, value: DivStroke) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn build(self) -> DivTextRangeBorder {
        DivTextRangeBorder {
            corner_radius: self.corner_radius,
            stroke: self.stroke,
        }
    }
}
