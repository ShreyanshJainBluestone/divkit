// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivTypedValue {
    #[serde(rename = "string")]
    StringValue(StringValue),
    #[serde(rename = "integer")]
    IntegerValue(IntegerValue),
    #[serde(rename = "number")]
    NumberValue(NumberValue),
    #[serde(rename = "color")]
    ColorValue(ColorValue),
    #[serde(rename = "boolean")]
    BooleanValue(BooleanValue),
    #[serde(rename = "url")]
    UrlValue(UrlValue),
    #[serde(rename = "dict")]
    DictValue(DictValue),
    #[serde(rename = "array")]
    ArrayValue(ArrayValue),
}
