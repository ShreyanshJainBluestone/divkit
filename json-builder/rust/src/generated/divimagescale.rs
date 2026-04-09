// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivImageScale {
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "no_scale")]
    NoScale,
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "stretch")]
    Stretch,
}
