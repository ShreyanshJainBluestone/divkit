// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionAnimatorStop {
    pub animator_id: String,
    #[serde(default = "DivActionAnimatorStop::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionAnimatorStopBuilder {
    pub animator_id: Option<String>,
    pub r#type: Option<String>,
}

impl DivActionAnimatorStop {
    pub fn builder() -> DivActionAnimatorStopBuilder {
        DivActionAnimatorStopBuilder::default()
    }
    pub fn default_type() -> String { "animator_stop".to_string() }
}

impl DivActionAnimatorStopBuilder {
    pub fn animator_id(mut self, value: String) -> Self {
        self.animator_id = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionAnimatorStop {
        DivActionAnimatorStop {
            animator_id: self.animator_id.expect("missing required field 'animator_id'"),
            r#type: String::from("animator_stop"),
        }
    }
}
