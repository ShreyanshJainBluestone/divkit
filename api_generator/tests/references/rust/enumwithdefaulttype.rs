// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]#[serde(tag = "type")]pub enum EnumWithDefaultType {    #[serde(rename = "with_default")]    WithDefault(WithDefault),    #[serde(rename = "without_default")]    WithoutDefault(WithoutDefault),}
