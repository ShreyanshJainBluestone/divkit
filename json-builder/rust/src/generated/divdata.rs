// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<DivFunction>>,
    pub log_id: String,
    pub states: Vec<DivDataState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timers: Option<Vec<DivTimer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_animation_selector: Option<Expression<DivTransitionSelector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_triggers: Option<Vec<DivTrigger>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<DivVariable>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivDataBuilder {
    pub functions: Option<Vec<DivFunction>>,
    pub log_id: Option<String>,
    pub states: Option<Vec<DivDataState>>,
    pub timers: Option<Vec<DivTimer>>,
    pub transition_animation_selector: Option<Expression<DivTransitionSelector>>,
    pub variable_triggers: Option<Vec<DivTrigger>>,
    pub variables: Option<Vec<DivVariable>>,
}

impl DivData {
    pub fn builder() -> DivDataBuilder {
        DivDataBuilder::default()
    }
    pub fn default_transition_animation_selector() -> Expression<DivTransitionSelector> { Expression::value(serde_json::from_str(r#""none""#).unwrap()) }
}

impl DivDataBuilder {
    pub fn functions(mut self, value: Vec<DivFunction>) -> Self {
        self.functions = Some(value);
        self
    }
    pub fn log_id(mut self, value: String) -> Self {
        self.log_id = Some(value);
        self
    }
    pub fn states(mut self, value: Vec<DivDataState>) -> Self {
        self.states = Some(value);
        self
    }
    pub fn timers(mut self, value: Vec<DivTimer>) -> Self {
        self.timers = Some(value);
        self
    }
    pub fn transition_animation_selector(mut self, value: Expression<DivTransitionSelector>) -> Self {
        self.transition_animation_selector = Some(value);
        self
    }
    pub fn variable_triggers(mut self, value: Vec<DivTrigger>) -> Self {
        self.variable_triggers = Some(value);
        self
    }
    pub fn variables(mut self, value: Vec<DivVariable>) -> Self {
        self.variables = Some(value);
        self
    }
    pub fn build(self) -> DivData {
        DivData {
            functions: self.functions,
            log_id: self.log_id.expect("missing required field 'log_id'"),
            states: self.states.expect("missing required field 'states'"),
            timers: self.timers,
            transition_animation_selector: self.transition_animation_selector.or_else(|| Some(DivData::default_transition_animation_selector())),
            variable_triggers: self.variable_triggers,
            variables: self.variables,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivDataState {
    pub div: Div,
    pub state_id: i64,
}

#[derive(Debug, Clone, Default)]
pub struct DivDataStateBuilder {
    pub div: Option<Div>,
    pub state_id: Option<i64>,
}

impl DivDataState {
    pub fn builder() -> DivDataStateBuilder {
        DivDataStateBuilder::default()
    }
}

impl DivDataStateBuilder {
    pub fn div(mut self, value: Div) -> Self {
        self.div = Some(value);
        self
    }
    pub fn state_id(mut self, value: i64) -> Self {
        self.state_id = Some(value);
        self
    }
    pub fn build(self) -> DivDataState {
        DivDataState {
            div: self.div.expect("missing required field 'div'"),
            state_id: self.state_id.expect("missing required field 'state_id'"),
        }
    }
}
