// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivAspect {
    pub ratio: Expression<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivAspectBuilder {
    pub ratio: Option<Expression<f64>>,
}

impl DivAspect {
    pub fn builder() -> DivAspectBuilder {
        DivAspectBuilder::default()
    }
}

impl DivAspectBuilder {
    pub fn ratio(mut self, value: Expression<f64>) -> Self {
        self.ratio = Some(value);
        self
    }
    pub fn build(self) -> DivAspect {
        DivAspect {
            ratio: self.ratio.expect("missing required field 'ratio'"),
        }
    }
}
