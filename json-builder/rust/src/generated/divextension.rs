// Generated code. Do not modify.

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::generated::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DivExtension {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct DivExtensionBuilder {
    pub id: Option<String>,
    pub params: Option<serde_json::Value>,
}

impl DivExtension {
    pub fn builder() -> DivExtensionBuilder {
        DivExtensionBuilder::default()
    }
}

impl DivExtensionBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn params(mut self, value: serde_json::Value) -> Self {
        self.params = Some(value);
        self
    }
    pub fn build(self) -> DivExtension {
        DivExtension {
            id: self.id.expect("missing required field 'id'"),
            params: self.params,
        }
    }
}
