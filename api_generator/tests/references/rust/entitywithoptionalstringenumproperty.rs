// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithOptionalStringEnumProperty {    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Expression<Property>>,    pub r#type: String,}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub enum EntityWithOptionalStringEnumPropertyProperty {    #[serde(rename = "first")]    First,    #[serde(rename = "second")]    Second,}
