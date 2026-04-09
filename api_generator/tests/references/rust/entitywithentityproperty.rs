// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]pub struct EntityWithEntityProperty {    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,    pub r#type: String,}
