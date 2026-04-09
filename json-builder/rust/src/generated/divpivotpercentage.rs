// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivPivotPercentage {
    #[serde(default = "DivPivotPercentage::default_type")]
    pub r#type: String,
    pub value: Expression<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct DivPivotPercentageBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<f64>>,
}

impl DivPivotPercentage {
    pub fn builder() -> DivPivotPercentageBuilder {
        DivPivotPercentageBuilder::default()
    }
    pub fn default_type() -> String { "pivot-percentage".to_string() }
}

impl DivPivotPercentageBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<f64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> DivPivotPercentage {
        DivPivotPercentage {
            r#type: String::from("pivot-percentage"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
