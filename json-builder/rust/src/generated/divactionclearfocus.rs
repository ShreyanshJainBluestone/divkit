// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionClearFocus {
    #[serde(default = "DivActionClearFocus::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionClearFocusBuilder {
    pub r#type: Option<String>,
}

impl DivActionClearFocus {
    pub fn builder() -> DivActionClearFocusBuilder {
        DivActionClearFocusBuilder::default()
    }
    pub fn default_type() -> String { "clear_focus".to_string() }
}

impl DivActionClearFocusBuilder {
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionClearFocus {
        DivActionClearFocus {
            r#type: String::from("clear_focus"),
        }
    }
}
