// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivTextGradient {
    #[serde(rename = "gradient")]
    DivLinearGradient(DivLinearGradient),
    #[serde(rename = "radial_gradient")]
    DivRadialGradient(DivRadialGradient),
}
