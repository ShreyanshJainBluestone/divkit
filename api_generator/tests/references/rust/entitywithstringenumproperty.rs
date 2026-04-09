// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithStringEnumProperty {    pub property: Expression<Property>,    pub r#type: String,}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub enum EntityWithStringEnumPropertyProperty {    #[serde(rename = "first")]    First,    #[serde(rename = "second")]    Second,}
