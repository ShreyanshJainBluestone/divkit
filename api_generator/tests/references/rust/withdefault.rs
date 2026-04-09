// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct WithDefault {    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,}
