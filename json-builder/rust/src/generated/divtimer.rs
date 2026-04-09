// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivTimer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_actions: Option<Vec<DivAction>>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_interval: Option<Expression<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_variable: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DivTimerBuilder {
    pub duration: Option<Expression<i64>>,
    pub end_actions: Option<Vec<DivAction>>,
    pub id: Option<String>,
    pub tick_actions: Option<Vec<DivAction>>,
    pub tick_interval: Option<Expression<i64>>,
    pub value_variable: Option<String>,
}

impl DivTimer {
    pub fn builder() -> DivTimerBuilder {
        DivTimerBuilder::default()
    }
    pub fn default_duration() -> Expression<i64> { Expression::value(0i64) }
}

impl DivTimerBuilder {
    pub fn duration(mut self, value: Expression<i64>) -> Self {
        self.duration = Some(value);
        self
    }
    pub fn end_actions(mut self, value: Vec<DivAction>) -> Self {
        self.end_actions = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn tick_actions(mut self, value: Vec<DivAction>) -> Self {
        self.tick_actions = Some(value);
        self
    }
    pub fn tick_interval(mut self, value: Expression<i64>) -> Self {
        self.tick_interval = Some(value);
        self
    }
    pub fn value_variable(mut self, value: String) -> Self {
        self.value_variable = Some(value);
        self
    }
    pub fn build(self) -> DivTimer {
        DivTimer {
            duration: self.duration.or_else(|| Some(DivTimer::default_duration())),
            end_actions: self.end_actions,
            id: self.id.expect("missing required field 'id'"),
            tick_actions: self.tick_actions,
            tick_interval: self.tick_interval,
            value_variable: self.value_variable,
        }
    }
}
