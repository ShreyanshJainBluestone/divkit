// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionFocusElement {
    pub element_id: Expression<String>,
    #[serde(default = "DivActionFocusElement::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionFocusElementBuilder {
    pub element_id: Option<Expression<String>>,
    pub r#type: Option<String>,
}

impl DivActionFocusElement {
    pub fn builder() -> DivActionFocusElementBuilder {
        DivActionFocusElementBuilder::default()
    }
    pub fn default_type() -> String { "focus_element".to_string() }
}

impl DivActionFocusElementBuilder {
    pub fn element_id(mut self, value: Expression<String>) -> Self {
        self.element_id = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionFocusElement {
        DivActionFocusElement {
            element_id: self.element_id.expect("missing required field 'element_id'"),
            r#type: String::from("focus_element"),
        }
    }
}
