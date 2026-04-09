// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithPropertyWithDefaultValue {    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<Expression<i64>>,    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested: Option<Nested>,    pub r#type: String,    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Expression<String>>,}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithPropertyWithDefaultValueNested {    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<Expression<i64>>,    pub non_optional: Expression<String>,    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Expression<String>>,}
