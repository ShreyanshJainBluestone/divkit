// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivSize {
    #[serde(rename = "fixed")]
    DivFixedSize(DivFixedSize),
    #[serde(rename = "match_parent")]
    DivMatchParentSize(DivMatchParentSize),
    #[serde(rename = "wrap_content")]
    DivWrapContentSize(DivWrapContentSize),
}
