// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivVisibility {
    #[serde(rename = "visible")]
    Visible,
    #[serde(rename = "invisible")]
    Invisible,
    #[serde(rename = "gone")]
    Gone,
}
