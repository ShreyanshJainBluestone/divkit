// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DivEvaluableType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "datetime")]
    Datetime,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "dict")]
    Dict,
    #[serde(rename = "array")]
    Array,
}
