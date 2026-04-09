// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTrigger {
    pub actions: Vec<DivAction>,
    pub condition: Expression<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Expression<DivTriggerMode>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTriggerBuilder {
    pub actions: Option<Vec<DivAction>>,
    pub condition: Option<Expression<bool>>,
    pub mode: Option<Expression<DivTriggerMode>>,
}

impl DivTrigger {
    pub fn builder() -> DivTriggerBuilder {
        DivTriggerBuilder::default()
    }
    pub fn default_mode() -> Expression<DivTriggerMode> { Expression::value(serde_json::from_str(r#""on_condition""#).unwrap()) }
}

impl DivTriggerBuilder {
    pub fn actions(mut self, value: Vec<DivAction>) -> Self {
        self.actions = Some(value);
        self
    }
    pub fn condition(mut self, value: Expression<bool>) -> Self {
        self.condition = Some(value);
        self
    }
    pub fn mode(mut self, value: Expression<DivTriggerMode>) -> Self {
        self.mode = Some(value);
        self
    }
    pub fn build(self) -> DivTrigger {
        DivTrigger {
            actions: self.actions.expect("missing required field 'actions'"),
            condition: self.condition.expect("missing required field 'condition'"),
            mode: self.mode.or_else(|| Some(DivTrigger::default_mode())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivTriggerMode {
    #[serde(rename = "on_condition")]
    OnCondition,
    #[serde(rename = "on_variable")]
    OnVariable,
}
