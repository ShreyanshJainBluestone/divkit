// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivDownloadCallbacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_fail_actions: Option<Vec<DivAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_success_actions: Option<Vec<DivAction>>,
}

#[derive(Debug, Clone, Default)]
pub struct DivDownloadCallbacksBuilder {
    pub on_fail_actions: Option<Vec<DivAction>>,
    pub on_success_actions: Option<Vec<DivAction>>,
}

impl DivDownloadCallbacks {
    pub fn builder() -> DivDownloadCallbacksBuilder {
        DivDownloadCallbacksBuilder::default()
    }
}

impl DivDownloadCallbacksBuilder {
    pub fn on_fail_actions(mut self, value: Vec<DivAction>) -> Self {
        self.on_fail_actions = Some(value);
        self
    }
    pub fn on_success_actions(mut self, value: Vec<DivAction>) -> Self {
        self.on_success_actions = Some(value);
        self
    }
    pub fn build(self) -> DivDownloadCallbacks {
        DivDownloadCallbacks {
            on_fail_actions: self.on_fail_actions,
            on_success_actions: self.on_success_actions,
        }
    }
}
