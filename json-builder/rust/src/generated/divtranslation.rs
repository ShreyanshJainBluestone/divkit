// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum DivTranslation {
    #[serde(rename = "translation-fixed")]
    DivFixedTranslation(DivFixedTranslation),
    #[serde(rename = "translation-percentage")]
    DivPercentageTranslation(DivPercentageTranslation),
}
