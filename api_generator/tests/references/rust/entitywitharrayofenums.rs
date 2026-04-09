// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithArrayOfEnums {    pub items: Vec<Item>,    pub r#type: String,}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub enum EntityWithArrayOfEnumsItem {    #[serde(rename = "first")]    First,    #[serde(rename = "second")]    Second,}
