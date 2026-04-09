// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivBackground {
    #[serde(rename = "gradient")]
    DivLinearGradient(DivLinearGradient),
    #[serde(rename = "radial_gradient")]
    DivRadialGradient(DivRadialGradient),
    #[serde(rename = "image")]
    DivImageBackground(DivImageBackground),
    #[serde(rename = "solid")]
    DivSolidBackground(DivSolidBackground),
    #[serde(rename = "nine_patch_image")]
    DivNinePatchBackground(DivNinePatchBackground),
}
