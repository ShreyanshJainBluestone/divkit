// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithStringEnumPropertyWithDefaultValue {    pub r#type: String,    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Expression<Value>>,}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub enum EntityWithStringEnumPropertyWithDefaultValueValue {    #[serde(rename = "first")]    First,    #[serde(rename = "second")]    Second,    #[serde(rename = "third")]    Third,}
