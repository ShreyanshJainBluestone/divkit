// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivStrokeStyle {
    #[serde(rename = "solid")]
    DivStrokeStyleSolid(DivStrokeStyleSolid),
    #[serde(rename = "dashed")]
    DivStrokeStyleDashed(DivStrokeStyleDashed),
}
