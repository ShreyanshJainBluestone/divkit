// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPoint {
    pub x: DivDimension,
    pub y: DivDimension,
}

#[derive(Debug, Clone, Default)]
pub struct DivPointBuilder {
    pub x: Option<DivDimension>,
    pub y: Option<DivDimension>,
}

impl DivPoint {
    pub fn builder() -> DivPointBuilder {
        DivPointBuilder::default()
    }
}

impl DivPointBuilder {
    pub fn x(mut self, value: DivDimension) -> Self {
        self.x = Some(value);
        self
    }
    pub fn y(mut self, value: DivDimension) -> Self {
        self.y = Some(value);
        self
    }
    pub fn build(self) -> DivPoint {
        DivPoint {
            x: self.x.expect("missing required field 'x'"),
            y: self.y.expect("missing required field 'y'"),
        }
    }
}
