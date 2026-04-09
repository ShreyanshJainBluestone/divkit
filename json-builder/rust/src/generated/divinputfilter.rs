// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivInputFilter {
    #[serde(rename = "regex")]
    DivInputFilterRegex(DivInputFilterRegex),
    #[serde(rename = "expression")]
    DivInputFilterExpression(DivInputFilterExpression),
}
