// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivAppearanceTransition {
    #[serde(rename = "set")]
    DivAppearanceSetTransition(DivAppearanceSetTransition),
    #[serde(rename = "fade")]
    DivFadeTransition(DivFadeTransition),
    #[serde(rename = "scale")]
    DivScaleTransition(DivScaleTransition),
    #[serde(rename = "slide")]
    DivSlideTransition(DivSlideTransition),
}
