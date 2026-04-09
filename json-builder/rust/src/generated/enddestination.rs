// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDestination {
    #[serde(default = "EndDestination::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct EndDestinationBuilder {
    pub r#type: Option<String>,
}

impl EndDestination {
    pub fn builder() -> EndDestinationBuilder {
        EndDestinationBuilder::default()
    }
    pub fn default_type() -> String { "end".to_string() }
}

impl EndDestinationBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> EndDestination {
        EndDestination {
            r#type: String::from("end"),
        }
    }
}
