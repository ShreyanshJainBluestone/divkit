// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithSimpleProperties {    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<Expression<bool>>,    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_int: Option<Expression<bool>>,    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Expression<String>>,    #[serde(skip_serializing_if = "Option::is_none")]
    pub double: Option<Expression<f64>>,    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer: Option<Expression<i64>>,    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_integer: Option<Expression<i64>>,    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<Expression<String>>,    pub r#type: String,    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Expression<String>>,}
