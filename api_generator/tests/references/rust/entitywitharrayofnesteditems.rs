// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithArrayOfNestedItems {    pub items: Vec<Item>,    pub r#type: String,}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithArrayOfNestedItemsItem {    pub entity: Entity,    pub property: Expression<String>,}
