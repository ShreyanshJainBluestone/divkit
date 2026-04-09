// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivAnimationInterpolator {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "ease")]
    Ease,
    #[serde(rename = "ease_in")]
    EaseIn,
    #[serde(rename = "ease_out")]
    EaseOut,
    #[serde(rename = "ease_in_out")]
    EaseInOut,
    #[serde(rename = "spring")]
    Spring,
}
