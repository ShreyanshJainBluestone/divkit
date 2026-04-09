// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithRawArray {    pub array: Expression<Vec<serde_json::Value>>,    pub r#type: String,}
