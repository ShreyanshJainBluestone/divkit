// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivActionTimer {
    pub action: Expression<DivActionTimerAction>,
    pub id: Expression<String>,
    #[serde(default = "DivActionTimer::default_type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Default)]
pub struct DivActionTimerBuilder {
    pub action: Option<Expression<DivActionTimerAction>>,
    pub id: Option<Expression<String>>,
    pub r#type: Option<String>,
}

impl DivActionTimer {
    pub fn builder() -> DivActionTimerBuilder {
        DivActionTimerBuilder::default()
    }
    pub fn default_type() -> String { "timer".to_string() }
}

impl DivActionTimerBuilder {
    pub fn action(mut self, value: Expression<DivActionTimerAction>) -> Self {
        self.action = Some(value);
        self
    }
    pub fn id(mut self, value: Expression<String>) -> Self {
        self.id = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn build(self) -> DivActionTimer {
        DivActionTimer {
            action: self.action.expect("missing required field 'action'"),
            id: self.id.expect("missing required field 'id'"),
            r#type: String::from("timer"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivActionTimerAction {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "resume")]
    Resume,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "reset")]
    Reset,
}
