// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StartDestination {
    #[serde(default = "StartDestination::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct StartDestinationBuilder {
    pub r#type: Option<String>,
}

impl StartDestination {
    pub fn builder() -> StartDestinationBuilder {
        StartDestinationBuilder::default()
    }
    pub fn default_type() -> String { "start".to_string() }
}

impl StartDestinationBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> StartDestination {
        StartDestination {
            r#type: String::from("start"),
        }
    }
}
