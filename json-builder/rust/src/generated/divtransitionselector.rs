// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivTransitionSelector {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "data_change")]
    DataChange,
    #[serde(rename = "state_change")]
    StateChange,
    #[serde(rename = "any_change")]
    AnyChange,
}
