// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionSetState {
    pub state_id: Expression<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<Expression<bool>>,
    #[serde(default = "DivActionSetState::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionSetStateBuilder {
    pub state_id: Option<Expression<String>>,
    pub temporary: Option<Expression<bool>>,
    pub r#type: Option<String>,
}

impl DivActionSetState {
    pub fn builder() -> DivActionSetStateBuilder {
        DivActionSetStateBuilder::default()
    }
    pub fn default_temporary() -> Expression<bool> { Expression::value(true) }
    pub fn default_type() -> String { "set_state".to_string() }
}

impl DivActionSetStateBuilder {
    pub fn state_id(mut self, value: Expression<String>) -> Self {
        self.state_id = Some(value);
        self
    }
    pub fn temporary(mut self, value: Expression<bool>) -> Self {
        self.temporary = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionSetState {
        DivActionSetState {
            state_id: self.state_id.expect("missing required field 'state_id'"),
            temporary: self.temporary.or_else(|| Some(DivActionSetState::default_temporary())),
            r#type: String::from("set_state"),
        }
    }
}
