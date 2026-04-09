// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithComplexPropertyWithDefaultValue {    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Property>,    pub r#type: String,}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithComplexPropertyWithDefaultValueProperty {    pub value: Expression<String>,}
