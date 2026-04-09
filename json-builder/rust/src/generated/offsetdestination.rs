// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OffsetDestination {
    #[serde(default = "OffsetDestination::default_type")]
    pub r#type: String,
    pub value: Expression<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct OffsetDestinationBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<i64>>,
}

impl OffsetDestination {
    pub fn builder() -> OffsetDestinationBuilder {
        OffsetDestinationBuilder::default()
    }
    pub fn default_type() -> String { "offset".to_string() }
}

impl OffsetDestinationBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<i64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> OffsetDestination {
        OffsetDestination {
            r#type: String::from("offset"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
