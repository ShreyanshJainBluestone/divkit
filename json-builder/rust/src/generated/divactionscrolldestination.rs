// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivActionScrollDestination {
    #[serde(rename = "offset")]
    OffsetDestination(OffsetDestination),
    #[serde(rename = "index")]
    IndexDestination(IndexDestination),
    #[serde(rename = "start")]
    StartDestination(StartDestination),
    #[serde(rename = "end")]
    EndDestination(EndDestination),
}
