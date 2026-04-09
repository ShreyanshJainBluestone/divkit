// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivAnimationDirection {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "reverse")]
    Reverse,
    #[serde(rename = "alternate")]
    Alternate,
    #[serde(rename = "alternate_reverse")]
    AlternateReverse,
}
