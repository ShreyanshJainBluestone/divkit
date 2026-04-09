// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivAppearanceSetTransition {
    pub items: Vec<DivAppearanceTransition>,
    #[serde(default = "DivAppearanceSetTransition::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivAppearanceSetTransitionBuilder {
    pub items: Option<Vec<DivAppearanceTransition>>,
    pub r#type: Option<String>,
}

impl DivAppearanceSetTransition {
    pub fn builder() -> DivAppearanceSetTransitionBuilder {
        DivAppearanceSetTransitionBuilder::default()
    }
    pub fn default_type() -> String { "set".to_string() }
}

impl DivAppearanceSetTransitionBuilder {
    pub fn items(mut self, value: Vec<DivAppearanceTransition>) -> Self {
        self.items = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivAppearanceSetTransition {
        DivAppearanceSetTransition {
            items: self.items.expect("missing required field 'items'"),
            r#type: String::from("set"),
        }
    }
}
