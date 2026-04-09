// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivInputValidator {
    #[serde(rename = "regex")]
    DivInputValidatorRegex(DivInputValidatorRegex),
    #[serde(rename = "expression")]
    DivInputValidatorExpression(DivInputValidatorExpression),
}
