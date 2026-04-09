// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivAnimator {
    #[serde(rename = "color_animator")]
    DivColorAnimator(DivColorAnimator),
    #[serde(rename = "number_animator")]
    DivNumberAnimator(DivNumberAnimator),
}
