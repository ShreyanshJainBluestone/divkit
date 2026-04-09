// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivTextRangeMask {
    #[serde(rename = "particles")]
    DivTextRangeMaskParticles(DivTextRangeMaskParticles),
    #[serde(rename = "solid")]
    DivTextRangeMaskSolid(DivTextRangeMaskSolid),
}
