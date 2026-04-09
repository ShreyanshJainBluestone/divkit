// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IndexDestination {
    #[serde(default = "IndexDestination::default_type")]
    pub r#type: String,
    pub value: Expression<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct IndexDestinationBuilder {
    pub r#type: Option<String>,
    pub value: Option<Expression<i64>>,
}

impl IndexDestination {
    pub fn builder() -> IndexDestinationBuilder {
        IndexDestinationBuilder::default()
    }
    pub fn default_type() -> String { "index".to_string() }
}

impl IndexDestinationBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn value(mut self, value: Expression<i64>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> IndexDestination {
        IndexDestination {
            r#type: String::from("index"),
            value: self.value.expect("missing required field 'value'"),
        }
    }
}
