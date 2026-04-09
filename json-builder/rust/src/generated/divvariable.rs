// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivVariable {
    #[serde(rename = "string")]
    StringVariable(StringVariable),
    #[serde(rename = "number")]
    NumberVariable(NumberVariable),
    #[serde(rename = "integer")]
    IntegerVariable(IntegerVariable),
    #[serde(rename = "boolean")]
    BooleanVariable(BooleanVariable),
    #[serde(rename = "color")]
    ColorVariable(ColorVariable),
    #[serde(rename = "url")]
    UrlVariable(UrlVariable),
    #[serde(rename = "dict")]
    DictVariable(DictVariable),
    #[serde(rename = "array")]
    ArrayVariable(ArrayVariable),
    #[serde(rename = "property")]
    PropertyVariable(PropertyVariable),
}
