// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivInputMask {
    #[serde(rename = "fixed_length")]
    DivFixedLengthInputMask(DivFixedLengthInputMask),
    #[serde(rename = "currency")]
    DivCurrencyInputMask(DivCurrencyInputMask),
    #[serde(rename = "phone")]
    DivPhoneInputMask(DivPhoneInputMask),
}
