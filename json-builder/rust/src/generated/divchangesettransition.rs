// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivChangeSetTransition {
    pub items: Vec<DivChangeTransition>,
    #[serde(default = "DivChangeSetTransition::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivChangeSetTransitionBuilder {
    pub items: Option<Vec<DivChangeTransition>>,
    pub r#type: Option<String>,
}

impl DivChangeSetTransition {
    pub fn builder() -> DivChangeSetTransitionBuilder {
        DivChangeSetTransitionBuilder::default()
    }
    pub fn default_type() -> String { "set".to_string() }
}

impl DivChangeSetTransitionBuilder {
    pub fn items(mut self, value: Vec<DivChangeTransition>) -> Self {
        self.items = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivChangeSetTransition {
        DivChangeSetTransition {
            items: self.items.expect("missing required field 'items'"),
            r#type: String::from("set"),
        }
    }
}
